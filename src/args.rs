use clap::Parser;

/// The `BrainFuck` interpreter.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The file to interpret.
    #[arg(short, long)]
    pub file: String,
}
