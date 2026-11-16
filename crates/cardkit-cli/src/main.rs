//! Command-line entry point for `cardkit`.
//!
//! The CLI is a thin adapter around the library. Keeping validation and data
//! loading in `cardkit` makes the same behavior available to native apps, web
//! services, and future WASM bindings.

#![forbid(unsafe_code)]

use std::process::ExitCode;

use cardkit::{CardkitError, Pan};
use clap::{Parser, Subcommand};

/// Validate payment card numbers without exposing them in logs.
#[derive(Debug, Parser)]
#[command(
    name = "cardkit",
    version,
    about = "PAN validation and BIN toolkit",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Commands exposed by the Phase 01 CLI.
#[derive(Debug, Subcommand)]
enum Command {
    /// Validate a card number and report its brand.
    ///
    /// The number may be written with spaces or hyphens. Quote it in the shell
    /// so those separators reach the program:
    ///
    ///   cardkit validate "4111 1111 1111 1111"
    Validate {
        /// Card number to validate.
        #[arg(value_name = "NUMBER")]
        number: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        // Error text must never contain the raw PAN. `CardkitError` follows
        // this rule; future CLI errors should follow it too.
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Dispatches a parsed command.
fn run(cli: Cli) -> Result<(), CardkitError> {
    match cli.command {
        Command::Validate { number } => {
            // Parsing validates the PAN. The rest of the output is a placeholder
            // because Phase 01 implementation has not started yet.
            let _pan: Pan = number.parse()?;

            // TODO(phase-01): print the masked PAN and detected brand, for
            // example: `valid  Visa  4111 **** **** 1111`.
            //
            // Do not print `_pan` directly if its Display implementation is
            // ever changed: call a dedicated masking helper or rely on the
            // guaranteed-masked Display contract.
            todo!("Phase 01: print validation result")
        }
    }
}
