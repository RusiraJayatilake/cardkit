//! Static BIN/IIN lookup data.
//!
//! BIN (Bank Identification Number) is a legacy name for the leading digits of
//! a PAN; ISO/IEC 7812 now calls this the Issuer Identification Number (IIN).
//! The table is intentionally loaded at runtime so it can be updated without
//! rebuilding the crate.
//!
//! Phase 01 uses CSV because it is easy to inspect and edit. A JSON loader can
//! be added later behind another feature flag.

use alloc::string::String;
use alloc::vec::Vec;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;

use crate::brand::CardBrand;
use crate::error::Result;

/// One row from the BIN data file.
///
/// The `iin` field stays a `String` so leading zeroes are preserved.
#[derive(Debug, Clone, Deserialize)]
pub struct BinRecord {
    /// Issuer identification number (the PAN prefix).
    pub iin: String,
    /// Card network associated with this prefix.
    pub brand: CardBrand,
    /// Human-readable issuer name.
    pub issuer: String,
    /// ISO 3166-1 alpha-2 country code, or a clearly marked test value.
    pub country: String,
}

/// An immutable collection of BIN records indexed by prefix.
///
/// A real implementation should sort records by descending IIN length so the
/// longest matching prefix wins (for example, an 8-digit IIN must beat a
/// 6-digit IIN).
#[derive(Debug, Default, Clone)]
pub struct BinTable {
    records: Vec<BinRecord>,
}

impl BinTable {
    /// Loads a BIN table from a CSV file.
    ///
    /// The CSV must have a header row with `iin,brand,issuer,country`.
    pub fn from_csv_path(_path: impl AsRef<Path>) -> Result<Self> {
        // TODO(phase-01): open the file and delegate to `from_csv_reader`.
        todo!("Phase 01: load BIN records from a CSV path")
    }

    /// Loads BIN records from any reader that yields CSV bytes.
    ///
    /// This constructor is useful for tests, embedded data, and future WASM
    /// callers that already have the file contents in memory.
    pub fn from_csv_reader<R: Read>(_reader: R) -> Result<Self> {
        // TODO(phase-01): use `csv::Reader` with serde support, collect the
        // records, and validate that the required columns are present.
        todo!("Phase 01: parse BIN records from CSV")
    }

    /// Returns the record with the longest IIN prefix matching `pan`.
    ///
    /// `pan` should contain digits only. Returns `None` when no row matches.
    pub fn lookup(&self, _pan: &str) -> Option<&BinRecord> {
        // TODO(phase-01): find the longest matching prefix.
        todo!("Phase 01: implement longest-prefix BIN lookup")
    }

    /// Number of loaded BIN records.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns `true` when the table contains no records.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn loads_the_bundled_sample_table() {
        let csv = include_str!("../data/bins.csv");
        let table = BinTable::from_csv_reader(csv.as_bytes()).expect("valid sample CSV");
        assert!(!table.is_empty());
    }

    #[test]
    #[ignore = "Phase 01: implementation pending"]
    fn lookup_prefers_the_longest_prefix() {
        let csv = concat!(
            "iin,brand,issuer,country\n",
            "400000,visa,Short Prefix,US\n",
            "40000012,visa,Long Prefix,US\n",
        );
        let table = BinTable::from_csv_reader(csv.as_bytes()).expect("valid CSV");

        assert_eq!(
            table
                .lookup("4000001234567890")
                .map(|row| row.issuer.as_str()),
            Some("Long Prefix")
        );
    }
}
