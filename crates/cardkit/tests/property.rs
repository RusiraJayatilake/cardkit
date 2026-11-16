//! Property tests for invariants that should hold for every generated PAN.
//!
//! Property tests complement example-based unit tests: instead of testing one
//! hard-coded number, proptest generates many inputs and shrinks failures to a
//! minimal counterexample.
//!
//! These tests are ignored until the Phase 01 implementation is complete.

use cardkit::{Pan, luhn};
use proptest::prelude::*;

proptest! {
    /// If a generated body is given a valid Luhn check digit, validation must
    /// succeed and parsing must preserve the normalized digits.
    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn appended_check_digit_produces_a_valid_pan(_body in "[0-9]{12,18}") {
        // TODO(phase-01): compute the check digit, append it, then assert
        // `luhn::validate` and `Pan::parse` both succeed.
        let _ = (luhn::validate, Pan::parse);
        prop_assert!(false, "Phase 01: generate valid PANs for property testing");
    }

    /// Masking must never reveal the middle digits of a valid 16-digit PAN.
    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn masked_display_hides_middle_digits(_digits in "[1-9][0-9]{15}") {
        // TODO(phase-01): construct a valid PAN, format it with `Display`,
        // and assert that its interior digits are replaced by `*`.
        prop_assert!(false, "Phase 01: property-test masked formatting");
    }
}
