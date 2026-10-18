//! `tls` — Handl stdlib port contract.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// `index.hnl` — the `tls` contract.
pub const TLS_HNL: &str = include_str!("index.hnl");
