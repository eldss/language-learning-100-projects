use clap::Parser;

/// Simple command line tool to calculate BMI from height and weight.
/// Uses metric values by default, but allows imperial units as well.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Height (meters or inches)
    pub height: f32,

    /// Weight (kg or lbs)
    pub weight: f32,

    /// Use imperial units (inches and pounds)
    #[arg(short, long)]
    pub imperial: bool,
}
