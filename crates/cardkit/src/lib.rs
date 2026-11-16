//! `cardkit` is a small toolkit for working with payment card numbers (PANs).
//!
//! # Phase 01 scope
//!
//! The public API is scaffolded here, but the algorithms are intentionally
//! left as `todo!()` placeholders:
//!
//! - Luhn checksum validation ([`luhn::validate`]).
//! - Brand detection by prefix and length ([`brand::detect_brand`]).
//! - A [`Pan`] newtype whose `Debug` and `Display` output is always masked.
//! - A static BIN table loaded from CSV ([`BinTable`], available with `std`).
//!
//! # Safety and privacy
//!
//! Real PANs are sensitive authentication data. Never log, print, or include a
//! full PAN in an error message. Prefer [`Pan`]'s masked `Debug`/`Display`
//! output, and call [`Pan::expose_digits`] only at the exact boundary where the
//! raw value is required (for example, a payment gateway request).
//!
//! # Feature flags
//!
//! - `std` (default): enables CSV-backed BIN data and `std::error::Error`.
//! - Without default features, the core types remain available in `no_std`
//!   environments that provide `alloc`.
//!
//! ```text
//! cargo check -p cardkit --no-default-features
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature = "std")]
pub mod bin_table;
pub mod brand;
pub mod error;
pub mod luhn;
pub mod pan;

#[cfg(feature = "std")]
pub use bin_table::{BinRecord, BinTable};
pub use brand::{CardBrand, detect_brand};
pub use error::{CardkitError, Result};
pub use luhn::validate as validate_luhn;
pub use pan::Pan;
