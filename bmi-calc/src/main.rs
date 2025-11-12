use clap::Parser;
use std::process::ExitCode;

mod bmi;
mod cli;

use crate::bmi::{CalculationError, calculate_bmi};
use crate::cli::Cli;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), CalculationError> {
    let args = Cli::parse();
    let bmi = calculate_bmi(args.height, args.weight, args.imperial)?;
    println!("BMI: {:.2}", bmi);
    Ok(())
}
