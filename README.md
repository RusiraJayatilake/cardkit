# cardkit

Card Number Validation & BIN Toolkit — a beginner-friendly Rust project for
learning enums, pattern matching, newtypes, custom errors, traits, testing,
`no_std`, and WASM-friendly library design.

> *Note!!*
> **Status: Phase 01 scaffold.** The API and module boundaries are in place, but
> the algorithms are deliberately `todo!()` placeholders. Nothing in this
> crate should be used to accept real payments yet.

## Project idea

`cardkit` provides reusable primitives for card-number handling:

- Luhn checksum validation (ISO/IEC 7812 check digit).
- Brand detection from a PAN's prefix and total length.
- A `Pan` newtype that can only hold a validated number and whose `Debug` and
  `Display` implementations always mask it (`4111 **** **** 1111`).
- A static BIN/IIN table loaded from CSV for issuer lookup.
- A `cardkit validate <NUMBER>` CLI.

The core validation rules stay in the library. The CLI is a thin adapter, so
the same code can later be exposed through a web service or a WASM binding.

## Workspace layout

```text
.
├── Cargo.toml                    # Workspace + shared dependency versions
├── crates/
│   ├── cardkit/                  # Reusable library crate
│   │   ├── Cargo.toml            # `std` feature gates CSV/BIN support
│   │   ├── data/bins.csv         # Illustrative BIN data
│   │   ├── src/
│   │   │   ├── lib.rs            # Public API and crate documentation
│   │   │   ├── brand.rs          # CardBrand + detect_brand
│   │   │   ├── luhn.rs           # Luhn checksum
│   │   │   ├── pan.rs            # Pan newtype + masked formatting
│   │   │   ├── bin_table.rs      # CSV-backed BIN table
│   │   │   └── error.rs          # CardkitError + Result alias
│   │   └── tests/property.rs     # proptest scaffolding
│   └── cardkit-cli/              # Binary crate
│       ├── Cargo.toml            # Binary name is `cardkit`
│       └── src/main.rs           # clap command skeleton
└── README.md
```

Separating the library and binary is the main Cargo design choice here:

- Library consumers do not pay for `clap`.
- The library can be compiled for `no_std` targets with
  `--no-default-features`.
- The CLI can change independently without changing the public library API.

## Build and run

```bash
# Check the whole workspace
cargo check --workspace

# Run the CLI skeleton
cargo run -- validate "4111 1111 1111 1111"

# Run unit, integration, and doc tests
cargo test --workspace

# Build API documentation
cargo doc -p cardkit --no-deps --open
```

The CLI command currently reaches a `todo!()` placeholder after parsing the
command line. That is intentional for this scaffold.

## Phase 01 implementation checklist

Search for `TODO(phase-01)` to find every placeholder.

- [ ] Normalize spaces/hyphens and enforce a 12–19 digit length in `Pan::parse`.
- [ ] Implement the Luhn checksum in `luhn::validate`.
- [ ] Implement longest-prefix, length-aware brand detection in
      `brand::detect_brand`.
- [ ] Implement masked `Display` and `Debug` for `Pan`; never derive `Debug`.
- [ ] Parse `data/bins.csv` and implement longest-prefix BIN lookup.
- [ ] Finish the CLI output using the masked representation only.
- [ ] Remove the `#[ignore]` attributes as tests start passing, then run
      `cargo test --workspace`.
- [ ] Run `cargo clippy --workspace --all-targets`.
- [ ] Document the supported brand rules and BIN data source.

## Security rules for contributors

PANs are sensitive. Treat every card number, valid or invalid, as production
data:

1. **Never log or print a full PAN.** Use `Pan`'s `Display`/`Debug` output.
2. **Never put a PAN in an error.** `CardkitError` stores only structural
   information such as length or an invalid byte offset.
3. **Never derive `Debug` for `Pan`.** A derived implementation would print the
   private string field.
4. **Expose raw digits only at the boundary.** `Pan::expose_digits` is an
   explicit, documented escape hatch for a payment gateway request—not for
   logging.
5. **Do not commit real BIN data or real card numbers.** The bundled CSV uses
   fictional issuers and well-known test values.

Luhn is a typo check, not authorization. A number that passes Luhn may still be
invalid, expired, or closed.

## BIN data format

`crates/cardkit/data/bins.csv` is a small illustrative table:

```csv
iin,brand,issuer,country
400000,visa,Example Bank,US
40000012,visa,Example Co-Brand,US
555555,mastercard,Example Bank,US
378282,amex,Example Bank,US
529741,mada,Example Bank,SA
621234,union_pay,Example Bank,CN
```

`iin` is a string so leading zeroes survive. Brand names map to the
`CardBrand` enum with `serde(rename_all = "snake_case")`. A production table
must come from a vetted provider; the bundled values are for scaffolding and
tests only.

## Testing strategy

- **Unit tests** live beside the code in `src/*.rs` and cover known examples
  and edge cases.
- **Doc tests** in the public API documentation keep usage examples honest.
- **Property tests** in `tests/property.rs` use `proptest` to generate many
  numbers and check invariants such as “adding a valid check digit produces a
  valid PAN” and “masked output never contains middle digits.”

The placeholders are marked `#[ignore]` so `cargo test` stays green before the
algorithms exist. Delete each `#[ignore]` with its implementation.

## `no_std` and WASM

The library has a default `std` feature. CSV loading uses `std`, while the core
types are `no_std` + `alloc` compatible:

```bash
# Core library without std (BIN table module is disabled)
cargo check -p cardkit --no-default-features

# WASM check (after installing the target)
rustup target add wasm32-unknown-unknown
cargo check -p cardkit --target wasm32-unknown-unknown --no-default-features
```

The CLI crate always uses `std` and is not intended to be compiled to WASM. A
future `cardkit-wasm` crate can expose the masked API through `wasm-bindgen`
without changing the core library.

### Credits
- Creator: Rusira Jayatilake
- Project Description: Card Number Validation & BIN Toolkit