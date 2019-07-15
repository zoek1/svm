#![deny(missing_docs)]
#![deny(unused)]

//! `svm-common` crate groups common shared code between the other `svm` crates

mod address;
mod default_key_hasher;
mod key_hasher;

/// Utility functions for messing mainly with bytes
pub mod utils;

pub use address::Address;
pub use default_key_hasher::DefaultKeyHasher;
pub use key_hasher::KeyHasher;