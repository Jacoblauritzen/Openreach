//! Gaussian Process regression with a `ConstantKernel * RBF` kernel — the
//! numerical core behind `BayesianQualifier` (`core/ml/qualifier.py`).
//!
//! This is a faithful reimplementation of the sklearn `Pipeline(StandardScaler,
//! GaussianProcessRegressor(kernel=C*RBF, alpha=0.1, n_restarts_optimizer=3))`
//! used by the original: same kernel, same noise `alpha`, same
//! log-marginal-likelihood objective, same predictive posterior (mean + std).
//!
//! It is **functionally** equivalent, not bit-identical: sklearn optimizes the
//! kernel hyperparameters with L-BFGS-B seeded by numpy's `RandomState`, whose
//! exact iterate sequence isn't reproducible outside numpy/scipy. We maximize the
//! same LML with analytic gradients and seeded random restarts, converging to the
//! same optimum for well-conditioned problems.

use nalgebra::{Cholesky, DMatrix, DVector};

/// Per-feature standardization — `sklearn.preprocessing.StandardScaler`.
#[derive(Debug, Clone)]
pub struct StandardScaler {
    pub mean: Vec<f64>,
    /// Per-feature scale; zero-variance features get scale 1 (as sklearn does).
    pub scale: Vec<f64>,
}

impl StandardScaler {
    pub fn fit(x: &DMatrix<f64>) -> StandardScaler {
        let (n, d) = (x.nrows(), x.ncols());
        let mut mean = vec![0.0; d];
        let mut scale = vec![1.0; d];
        if n == 0 {
            return StandardScaler { mean, scale };
        }
        for j in 0..d {
            let col = x.column(j);
            let m = col.sum() / n as f64;
            mean[j] = m;
            // population variance (ddof=0), matching sklearn.
            let var = col.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / n as f64;
            let s = var.sqrt();
            scale[j] = if s == 0.0 { 1.0 } else { s };
        }
        StandardScaler { mean, scale }
    }

    pub fn transform(&self, x: &DMatrix<f64>) -> DMatrix<f64> {
        let (n, d) = (x.nrows(), x.ncols());
        let mut out = x.clone();
        for i in 0..n {
            for j in 0..d {
                out[(i, j)] = (x[(i, j)] - self.mean[j]) / self.scale[j];
            }
        }
        out
    }

    pub fn transform_row(&self, x: &[f64]) -> Vec<f64> {
        x.iter()
            .enumerate()
            .map(|(j, v)| (v - self.mean[j]) / self.scale[j])
            .collect()
    }
}

/// Squared Euclidean distances between the rows of `a` (m×d) and `b` (n×d) → m×n.
fn sqdist(a: &DMatrix<f64>, b: &DMatrix<f64>) -> DMatrix<f64> {
    let (m, n) = (a.nrows(), b.nrows());
    let mut d = DMatrix::zeros(m, n);
    for i in 0..m {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..a.ncols() {
                let diff = a[(i, k)] - b[(j, k)];
                s += diff * diff;
            }
            d[(i, j)] = s;
        }
    }
    d
}

/// A fitted GP: kernel hyperparameters + the training-conditioned posterior state.
#[derive(Debug, Clone)]
pub struct Gp {
    /// `ConstantKernel` value `c` (signal variance).
    pub constant_value: f64,
    /// `RBF` length scale `l`.
    pub length_scale: f64,
    /// Observation noise added to the kernel diagonal (`alpha`).
    pub alpha: f64,
    x_train: DMatrix<f64>,
    /// `K^{-1} y` (the "alpha_" vector in sklearn).
    alpha_vec: DVector<f64>,
    /// Cholesky factor of `K = k(X,X) + alpha·I`.
    chol: Cholesky<f64, nalgebra::Dyn>,
    /// Log marginal likelihood at the chosen hyperparameters.
    pub log_marginal_likelihood: f64,
}

impl Gp {
    /// Fit on standardized `x` (n×d) and targets `y` (n), optimizing the kernel
    /// hyperparameters by LML with `n_restarts` seeded random restarts.
    pub fn fit(
        x: &DMatrix<f64>,
        y: &DVector<f64>,
        alpha: f64,
        length_scale_init: f64,
        n_restarts: usize,
        rng: &mut impl rand::Rng,
    ) -> Option<Gp> {
        let d = sqdist(x, x);
        // theta = [ln c, ln l]; bounds ln(1e-5)..ln(1e5), as sklearn's defaults.
        let lo = (1e-5f64).ln();
        let hi = (1e5f64).ln();

        let mut starts: Vec<[f64; 2]> = vec![[0.0_f64.ln().max(lo), length_scale_init.ln()]];
        //                                     ln(constant=1)=0                ln(l0)
        starts[0][0] = 0.0; // ln(1.0)
        for _ in 0..n_restarts {
            let c = lo + (hi - lo) * rng.gen::<f64>();
            let l = lo + (hi - lo) * rng.gen::<f64>();
            starts.push([c, l]);
        }

        let mut best: Option<([f64; 2], f64)> = None;
        for start in starts {
            if let Some((theta, lml)) = maximize_lml(&d, y, alpha, start, lo, hi) {
                if best.as_ref().map(|(_, b)| lml > *b).unwrap_or(true) {
                    best = Some((theta, lml));
                }
            }
        }
        let (theta, lml) = best?;
        let c = theta[0].exp();
        let l = theta[1].exp();

        let k = kernel_from_dist(&d, c, l, alpha);
        let chol = Cholesky::new(k)?;
        let alpha_vec = chol.solve(y);

        Some(Gp {
            constant_value: c,
            length_scale: l,
            alpha,
            x_train: x.clone(),
            alpha_vec,
            chol,
            log_marginal_likelihood: lml,
        })
    }

    /// Posterior mean + std for each row of standardized `x_test`.
    /// (`GaussianProcessRegressor.predict(..., return_std=True)`)
    pub fn predict(&self, x_test: &DMatrix<f64>) -> (DVector<f64>, DVector<f64>) {
        let c = self.constant_value;
        let l = self.length_scale;
        let d_star = sqdist(x_test, &self.x_train); // (n_test × n_train)
        let k_star = d_star.map(|dd| c * (-dd / (2.0 * l * l)).exp());

        let mean = &k_star * &self.alpha_vec;

        // var_j = k(x*,x*) - k*_j^T K^{-1} k*_j, with k(x*,x*) = c.
        let n_test = x_test.nrows();
        let mut std = DVector::zeros(n_test);
        for j in 0..n_test {
            let kj = k_star.row(j).transpose(); // (n_train)
            let v = self.chol.solve(&kj); // K^{-1} k*_j
            let quad = kj.dot(&v);
            let var = (c - quad).max(0.0);
            std[j] = var.sqrt();
        }
        (mean, std)
    }
}

/// `K = c·exp(-D/(2 l²)) + alpha·I` from a precomputed squared-distance matrix.
fn kernel_from_dist(d: &DMatrix<f64>, c: f64, l: f64, alpha: f64) -> DMatrix<f64> {
    let n = d.nrows();
    let mut k = d.map(|dd| c * (-dd / (2.0 * l * l)).exp());
    for i in 0..n {
        k[(i, i)] += alpha;
    }
    k
}

/// LML and its gradient wrt `theta = [ln c, ln l]` at the given hyperparameters.
fn lml_and_grad(
    d: &DMatrix<f64>,
    y: &DVector<f64>,
    alpha: f64,
    theta: [f64; 2],
) -> Option<(f64, [f64; 2])> {
    let n = d.nrows();
    let c = theta[0].exp();
    let l = theta[1].exp();

    let k_signal = d.map(|dd| c * (-dd / (2.0 * l * l)).exp());
    let mut k = k_signal.clone();
    for i in 0..n {
        k[(i, i)] += alpha;
    }
    let chol = Cholesky::new(k)?;
    let a = chol.solve(y); // K^{-1} y
    // logdet(K) = 2 Σ ln(diag(L))
    let l_factor = chol.l();
    let mut logdet = 0.0;
    for i in 0..n {
        logdet += l_factor[(i, i)].ln();
    }
    logdet *= 2.0;
    let lml =
        -0.5 * y.dot(&a) - 0.5 * logdet - (n as f64) / 2.0 * (2.0 * std::f64::consts::PI).ln();

    // K^{-1} (explicit) for the gradient trace terms.
    let k_inv = chol.inverse();

    // dK/d(ln c) = K_signal ; dK/d(ln l) = K_signal ∘ (D / l²)
    let dk_dlnc = k_signal.clone();
    let dk_dlnl = {
        let mut m = k_signal.clone();
        for i in 0..n {
            for j in 0..n {
                m[(i, j)] *= d[(i, j)] / (l * l);
            }
        }
        m
    };

    let grad_c = 0.5 * (a.dot(&(&dk_dlnc * &a)) - trace_of_product(&k_inv, &dk_dlnc));
    let grad_l = 0.5 * (a.dot(&(&dk_dlnl * &a)) - trace_of_product(&k_inv, &dk_dlnl));
    Some((lml, [grad_c, grad_l]))
}

/// tr(A·B) without forming the full product.
fn trace_of_product(a: &DMatrix<f64>, b: &DMatrix<f64>) -> f64 {
    let n = a.nrows();
    let mut t = 0.0;
    for i in 0..n {
        for k in 0..n {
            t += a[(i, k)] * b[(k, i)];
        }
    }
    t
}

/// Maximize LML from a start point via projected gradient ascent with Armijo
/// backtracking. Returns `(theta*, lml*)`.
fn maximize_lml(
    d: &DMatrix<f64>,
    y: &DVector<f64>,
    alpha: f64,
    start: [f64; 2],
    lo: f64,
    hi: f64,
) -> Option<([f64; 2], f64)> {
    let clamp = |t: [f64; 2]| [t[0].clamp(lo, hi), t[1].clamp(lo, hi)];
    let mut theta = clamp(start);
    let (mut lml, mut grad) = lml_and_grad(d, y, alpha, theta)?;

    for _ in 0..200 {
        let gnorm = (grad[0] * grad[0] + grad[1] * grad[1]).sqrt();
        if gnorm < 1e-6 {
            break;
        }
        // Backtracking line search along the (ascent) gradient direction.
        let mut step = 1.0_f64.min(1.0 / gnorm);
        let mut improved = false;
        for _ in 0..30 {
            let cand = clamp([theta[0] + step * grad[0], theta[1] + step * grad[1]]);
            if let Some((cand_lml, cand_grad)) = lml_and_grad(d, y, alpha, cand) {
                if cand_lml > lml + 1e-4 * step * gnorm * gnorm {
                    theta = cand;
                    lml = cand_lml;
                    grad = cand_grad;
                    improved = true;
                    break;
                }
            }
            step *= 0.5;
        }
        if !improved {
            break;
        }
    }
    Some((theta, lml))
}

/// Standard normal CDF Φ(x) = ½(1 + erf(x/√2)).
pub fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Error function — Abramowitz & Stegun 7.1.26 (|error| ≤ 1.5e-7).
fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t
            * (-x * x).exp();
    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn scaler_standardizes() {
        let x = DMatrix::from_row_slice(3, 1, &[1.0, 2.0, 3.0]);
        let s = StandardScaler::fit(&x);
        assert!((s.mean[0] - 2.0).abs() < 1e-12);
        let t = s.transform(&x);
        // mean 0 after transform
        assert!((t.column(0).sum()).abs() < 1e-9);
    }

    #[test]
    fn gp_interpolates_training_points() {
        // 1-D separable data: negatives near 0, positives near 5.
        let x = DMatrix::from_row_slice(6, 1, &[0.0, 0.2, 0.1, 5.0, 4.8, 5.1]);
        let y = DVector::from_row_slice(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        let scaler = StandardScaler::fit(&x);
        let xs = scaler.transform(&x);
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let gp = Gp::fit(&xs, &y, 0.1, (1.0_f64).sqrt(), 3, &mut rng).expect("fit");

        // A point near the positive cluster should score higher than one near
        // the negative cluster.
        let lo = scaler.transform(&DMatrix::from_row_slice(1, 1, &[0.1]));
        let hi = scaler.transform(&DMatrix::from_row_slice(1, 1, &[5.0]));
        let (m_lo, s_lo) = gp.predict(&lo);
        let (m_hi, _s_hi) = gp.predict(&hi);
        assert!(m_hi[0] > m_lo[0], "hi {} !> lo {}", m_hi[0], m_lo[0]);
        assert!(s_lo[0] >= 0.0);
    }

    #[test]
    fn norm_cdf_reasonable() {
        assert!((norm_cdf(0.0) - 0.5).abs() < 1e-6);
        assert!(norm_cdf(3.0) > 0.99);
        assert!(norm_cdf(-3.0) < 0.01);
    }
}
\n// revival 2026 touch: src/ml/gp.rs\n\n// revival 2026 touch: src/ml/gp.rs\n
// revival 2026 update: src/ml/gp.rs
