//! FastEmbed text embedding — a port of `openhaze/core/ml/embeddings.py`.
//!
//! `embed_text` / `embed_texts` produce `EMBEDDING_DIM`-wide `f32` vectors for
//! `BAAI/bge-small-en-v1.5`, matching the original. The model is a lazily-loaded
//! singleton.
//!
//! Two backends:
//! - **`fastembed` feature on** — the real ONNX model (byte-for-byte the same
//!   embeddings as the Python original).
//! - **default (feature off)** — a deterministic hash-based stand-in of the right
//!   width, so the pipeline and GP have consistent vectors to work with in builds
//!   without the ONNX runtime. It carries no semantic signal and must not be used
//!   for real campaigns; a one-time warning is logged.

use crate::conf::EMBEDDING_DIM;

/// Embed a single text → `EMBEDDING_DIM`-dim vector. (`embed_text`)
pub fn embed_text(text: &str) -> Vec<f32> {
    backend::embed_texts(&[text.to_string()])
        .into_iter()
        .next()
        .unwrap_or_else(|| vec![0.0; EMBEDDING_DIM])
}

/// Embed many texts → one vector each. (`embed_texts`)
pub fn embed_texts(texts: &[String]) -> Vec<Vec<f32>> {
    backend::embed_texts(texts)
}

// ---------------------------------------------------------------------------
// Real FastEmbed backend (BGE-small-en-v1.5 via ort).
// ---------------------------------------------------------------------------
#[cfg(feature = "fastembed")]
mod backend {
    use std::sync::{Mutex, OnceLock};

    use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

    use crate::conf::fastembed_cache_dir;

    fn model() -> &'static Mutex<TextEmbedding> {
        static MODEL: OnceLock<Mutex<TextEmbedding>> = OnceLock::new();
        MODEL.get_or_init(|| {
            let cache = fastembed_cache_dir();
            let _ = std::fs::create_dir_all(&cache);
            tracing::debug!("loading embedding model: BAAI/bge-small-en-v1.5");
            let model = TextEmbedding::try_new(
                InitOptions::new(EmbeddingModel::BGESmallENV15).with_cache_dir(cache),
            )
            .expect("initialize FastEmbed BGE-small-en-v1.5");