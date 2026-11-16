//! The [`Pan`] newtype and its privacy-preserving formatting.
//!
//! The inner `String` is private so callers cannot accidentally bypass
//! validation or print raw digits. `Debug` and `Display` are implemented
//! manually (not derived) to guarantee that formatted output is masked.

use alloc::string::String;
use core::fmt;
use core::str::FromStr;

use crate::brand::{CardBrand, detect_brand};
use crate::error::{CardkitError, Result};

/// A syntactically valid Primary Account Number.
///
/// Construction is fallible and should perform all of the following:
///
/// 1. Remove the display separators that users commonly type (spaces and
///    hyphens).
/// 2. Reject empty or non-digit input.
/// 3. Enforce a reasonable PAN length (12-19 digits).
/// 4. Run the Luhn checksum.
/// 5. Detect a brand when possible; an unknown brand is not necessarily an
///    invalid PAN and is represented as `None`.
///
/// # Privacy
///
/// Both `Debug` and `Display` are always masked. The raw digits are available
/// only through [`Pan::expose_digits`].
///
/// ```no_run
/// use cardkit::Pan;
///
/// let pan: Pan = "4111 1111 1111 1111".parse()?;
/// assert_eq!(pan.to_string(), "4111 **** **** 1111");
/// # Ok::<(), cardkit::CardkitError>(())
/// ```
#[derive(Clone)]
pub struct Pan(String);

impl Pan {
    /// Validates and normalizes a card number.
    ///
    /// Accepts digits with optional ASCII spaces or hyphens. The error value
    /// must never contain the original input.
    pub fn parse(_input: &str) -> Result<Self> {
        // TODO(phase-01): normalize separators, validate length, run Luhn,
        // and store only the normalized digit string.
        todo!("Phase 01: implement Pan parsing and validation")
    }

    /// Returns the raw PAN digits.
    ///
    /// # Security warning
    ///
    /// Do not log or display this value. Use `Display` for any user-facing
    /// output and pass the exposed value only to code that needs the raw PAN.
    pub fn expose_digits(&self) -> &str {
        &self.0
    }

    /// Returns the detected brand, or `None` when no prefix rule matches.
    pub fn brand(&self) -> Option<CardBrand> {
        detect_brand(self.expose_digits())
    }
}

impl FromStr for Pan {
    type Err = CardkitError;

    fn from_str(input: &str) -> Result<Self> {
        Self::parse(input)
    }
}

impl fmt::Display for Pan {
    /// Formats the PAN with all but the first four and last four digits hidden.
    ///
    /// For a 16-digit Visa, the target output is:
    ///
    /// ```text
    /// 4111 **** **** 1111
    /// ```
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(phase-01): mask the middle digits and group the result.
        //
        // Keep the first four and last four digits, replace the rest with `*`,
        // and preserve the familiar four-digit grouping.
        todo!("Phase 01: implement masked Display for Pan")
    }
}

impl fmt::Debug for Pan {
    /// Mirrors the masked `Display` representation.
    ///
    /// This intentionally does not use `derive(Debug)`: a derived implementation
    /// would print the private field and leak the full PAN into logs.
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(phase-01): delegate to the masked formatter.
        todo!("Phase 01: implement masked Debug for Pan")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn parses_common_spacing_and_hyphens() {
        let spaced = Pan::parse("4111 1111 1111 1111").expect("valid PAN");
        let hyphenated = Pan::parse("4111-1111-1111-1111").expect("valid PAN");

        assert_eq!(spaced.expose_digits(), "4111111111111111");
        assert_eq!(hyphenated.expose_digits(), "4111111111111111");
    }

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn display_masks_the_account_number() {
        let pan = Pan::parse("4111111111111111").expect("valid PAN");
        assert_eq!(pan.to_string(), "4111 **** **** 1111");
    }

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn debug_never_contains_the_raw_account_number() {
        let pan = Pan::parse("4111111111111111").expect("valid PAN");
        assert!(!format!("{pan:?}").contains("4111111111111111"));
    }

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn rejects_a_bad_checksum() {
        assert!(Pan::parse("4111111111111112").is_err());
    }
}
