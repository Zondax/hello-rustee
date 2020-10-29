//! zkms-keystore
#![deny(
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

mod keystore;
pub use keystore::KeyStore;
