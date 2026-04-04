//! GP-regression qualifier with BALD active learning — a port of
//! `openhaze/core/ml/qualifier.py` (`BayesianQualifier`, `KitQualifier`, and
//! the numeric helpers). The LLM `qualify_with_llm` leg lands in phase 4.

use nalgebra::{DMatrix, DVector};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, StandardNormal};

use crate::conf::{EMBEDDING_DIM, QUALIFICATION_N_MC_SAMPLES};

use super::gp::{norm_cdf, Gp, StandardScaler};

/// Compact one-liner stats string for qualification logging. (`format_prediction`)
pub fn format_prediction(prob: f64, entropy: f64, std: f64, n_obs: usize) -> String {
    format!("P(f>0.5)={prob:.3}, entropy={entropy:.4}, std={std:.4}, obs={n_obs}")
}

/// `H(p) = -p ln p - (1-p) ln(1-p)`, safe for edge values. (`_binary_entropy`)
pub fn binary_entropy(p: f64) -> f64 {
    let p = p.clamp(1e-12, 1.0 - 1e-12);
    -p * p.ln() - (1.0 - p) * (1.0 - p).ln()
}

/// `P(f > 0.5)` from a Gaussian posterior `N(mean, std)`. (`_prob_above_half`)
pub fn prob_above_half(mean: f64, std: f64) -> f64 {
    if std < 1e-12 {
        if mean > 0.5 {
            1.0
        } else {
            0.0
        }
    } else {
        norm_cdf((mean - 0.5) / std)
    }
}

struct Fitted {
    scaler: StandardScaler,
    gp: Gp,
}

impl Fitted {
    /// Standardize then GP-predict; returns (mean, std) per row.
    fn predict(&self, embeddings: &[Vec<f64>]) -> (Vec<f64>, Vec<f64>) {
        let d = embeddings.first().map(|e| e.len()).unwrap_or(0);
        let mut m = DMatrix::zeros(embeddings.len(), d);
        for (i, e) in embeddings.iter().enumerate() {
            for (j, v) in e.iter().enumerate() {
                m[(i, j)] = *v;
            }
        }
        let xs = self.scaler.transform(&m);
        let (mean, std) = self.gp.predict(&xs);
        (mean.iter().copied().collect(), std.iter().copied().collect())
    }
}

/// Gaussian Process Regressor for active-learning qualification.
/// (`BayesianQualifier`)
pub struct BayesianQualifier {
    embedding_dim: usize,
    n_mc_samples: usize,
    x: Vec<Vec<f64>>,
    y: Vec<i32>,
    fitted: Option<Fitted>,
    dirty: bool,
    rng: StdRng,
}

/// Maximum ratio of majority-to-minority samples for GP fitting. (`_MAX_IMBALANCE_RATIO`)
const MAX_IMBALANCE_RATIO: usize = 2;
/// Observation noise on the kernel diagonal (`alpha=0.1` in the original).
const GP_ALPHA: f64 = 0.1;
/// L-BFGS-B restarts in sklearn; we mirror the count.
const N_RESTARTS: usize = 3;

impl BayesianQualifier {
    pub fn new(seed: u64) -> Self {
        BayesianQualifier {
            embedding_dim: EMBEDDING_DIM,
            n_mc_samples: QUALIFICATION_N_MC_SAMPLES,
            x: Vec::new(),
            y: Vec::new(),
            fitted: None,
            dirty: true,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn n_obs(&self) -> usize {
        self.y.len()
    }

    /// `(n_negatives, n_positives)`. (`class_counts`)
    pub fn class_counts(&self) -> (usize, usize) {
        let n_pos = self.y.iter().filter(|v| **v != 0).count();
        (self.y.len() - n_pos, n_pos)
    }

    /// Record a new labelled observation; the model is lazily re-fitted. (`update`)
    pub fn update(&mut self, embedding: &[f32], label: i32) {
        self.x.push(embedding.iter().map(|v| *v as f64).collect());
        self.y.push(label);
        self.dirty = true;
    }

    /// Bulk-load historical labels and fit once. (`warm_start`)
    pub fn warm_start(&mut self, embeddings: &[Vec<f32>], labels: &[i32]) {
        self.x = embeddings
            .iter()
            .map(|e| e.iter().map(|v| *v as f64).collect())
            .collect();
        self.y = labels.to_vec();
        self.dirty = true;
        if self.x.len() >= 2 {
            self.fit_if_needed();
        }
    }

    /// Fit StandardScaler + GPR if dirty and feasible; true when usable.\n// revival 2026 touch: src/ml/qualifier.rs\n\n// revival 2026 touch: src/ml/qualifier.rs\n