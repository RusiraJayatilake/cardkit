//! Card brand identification from a PAN's leading digits and total length.
//!
//! Prefix rules overlap in the real world (especially for co-branded Mada and
//! UnionPay cards), so BIN-table lookups should take precedence when available.

use core::fmt;

/// A payment card network recognized by `cardkit`.
///
/// This enum is deliberately small for Phase 01. Add new networks behind a
/// minor-version change as the BIN table grows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "snake_case"))]
pub enum CardBrand {
    /// Visa: commonly starts with `4` and is 13, 16, or 19 digits long.
    Visa,
    /// Mastercard: uses the `51`-`55` and `2221`-`2720` ranges, typically 16 digits.
    Mastercard,
    /// American Express: starts with `34` or `37` and is 15 digits long.
    Amex,
    /// Mada: Saudi domestic debit network with its own IIN ranges.
    Mada,
    /// UnionPay: commonly starts with `62` and is 16-19 digits long.
    UnionPay,
}

impl fmt::Display for CardBrand {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(phase-01): return the human-readable network name.
        //
        // Keep this as a `match` over all variants so adding a brand without
        // updating the display name is a compile-time error.
        todo!("Phase 01: implement Display for CardBrand")
    }
}

/// Detects the most likely brand from the supplied PAN digits.
///
/// The input should contain digits only (normalize separators before calling).
///
/// # Rules to implement
///
/// 1. Match the longest known prefix first. A longer IIN must not be beaten by
///    a shorter, more generic prefix.
/// 2. Check the total length after matching the prefix.
/// 3. Prefer a BIN-table entry when the caller has one; this function is the
///    fallback for unknown BINs.
///
/// # Examples
///
/// ```no_run
/// use cardkit::detect_brand;
///
/// assert_eq!(detect_brand("4111111111111111"), Some(cardkit::CardBrand::Visa));
/// ```
pub fn detect_brand(_digits: &str) -> Option<CardBrand> {
    // TODO(phase-01): implement longest-prefix and length checks.
    todo!("Phase 01: implement brand detection by prefix and length")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn detects_visa_prefix_and_length() {
        assert_eq!(detect_brand("4111111111111111"), Some(CardBrand::Visa));
    }

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn rejects_a_known_prefix_with_the_wrong_length() {
        // Amex is 15 digits; this 16-digit value must not be reported as Amex.
        assert_eq!(detect_brand("3782822463100050"), None);
    }
}
