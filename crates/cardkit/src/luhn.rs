//! Luhn (mod 10) checksum validation.
//!
//! Luhn catches most single-digit typos and many adjacent transpositions. It
//! is a checksum, **not** proof that a card exists or that it can be charged.
//! A full offline validation flow still needs length and brand checks.

use crate::error::Result;

/// Checks the Luhn checksum of a PAN.
///
/// `digits` must contain only ASCII decimal digits. The input is never stored
/// or logged by this function.
///
/// # Algorithm (ISO/IEC 7812 check digit)
///
/// Starting from the rightmost digit and moving left:
///
/// 1. Double every second digit (the check digit itself is not doubled).
/// 2. If doubling produces a value greater than 9, subtract 9.
/// 3. Sum all digits; a valid PAN has a sum divisible by 10.
///
/// # Examples
///
/// ```no_run
/// use cardkit::luhn;
///
/// assert!(luhn::validate("4111111111111111").is_ok());
/// assert!(luhn::validate("4111111111111112").is_err());
/// ```
pub fn validate(_digits: &str) -> Result<()> {
    // TODO(phase-01): implement the Luhn algorithm.
    //
    // Suggested steps:
    // 1. Reject an empty string and any non-ASCII-digit byte.
    // 2. Walk the bytes from right to left, doubling every second digit.
    // 3. Return `Ok(())` only when the total modulo 10 is zero.
    todo!("Phase 01: implement Luhn checksum validation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn accepts_a_known_valid_number() {
        assert!(validate("4111111111111111").is_ok());
    }

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn rejects_a_single_digit_typo() {
        assert!(validate("4111111111111112").is_err());
    }

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn rejects_non_digit_input() {
        assert!(validate("4111-1111-1111-1111").is_err());
    }
}
