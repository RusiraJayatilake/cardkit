//! Error types shared by the library and CLI.
//!
//! Errors deliberately avoid storing the rejected PAN. A malformed number is
//! still sensitive input, and error values frequently end up in logs.

use thiserror::Error;

/// Errors returned by `cardkit` operations.
#[derive(Debug, Error)]
pub enum CardkitError {
    /// The input did not contain any card-number digits.
    #[error("card number must not be empty")]
    Empty,

    /// The input contained a character that is not an ASCII digit.
    ///
    /// `position` is a byte offset, not a copy of the sensitive input.
    #[error("card number contains a non-digit character at position {position}")]
    NonDigit {
        /// Byte offset of the invalid character.
        position: usize,
    },

    /// The normalized number is outside the supported PAN length range.
    #[error("card number must contain between {min} and {max} digits (got {actual})")]
    InvalidLength {
        /// Shortest accepted PAN length.
        min: usize,
        /// Longest accepted PAN length.
        max: usize,
        /// Number of digits supplied after normalization.
        actual: usize,
    },

    /// The number does not satisfy the Luhn checksum.
    #[error("card number failed the Luhn checksum")]
    LuhnFailed,

    /// The BIN data file could not be opened or read.
    #[cfg(feature = "std")]
    #[error("could not read BIN data: {0}")]
    BinDataIo(#[from] std::io::Error),

    /// The BIN data file was not valid CSV.
    #[cfg(feature = "std")]
    #[error("could not parse BIN data: {0}")]
    BinDataCsv(#[from] csv::Error),
}

/// Convenience alias for `Result<T, CardkitError>`.
pub type Result<T> = core::result::Result<T, CardkitError>;
