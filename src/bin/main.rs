use clap::{Parser, Subcommand};
use ingest_demo_rs::{generator, ingestion};
use std::path::PathBuf;
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate random JSON records based on examples
    Generate {
        /// Number of records to generate
        #[arg(short, long, default_value_t = 10)]
        count: usize,

        /// Path to the example JSON file
        #[arg(short, long, default_value = "examples/test_gen.json")]
        example: PathBuf,
    },

    /// Convert JSON from stdin to partitioned Parquet files
    Convert {
        /// Output directory for Parquet files
        #[arg(short, long, default_value = "data")]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { count, example } => {
            info!(
                "Generating {} random records based on {}",
                count,
                example.display()
            );
            generator::generate_records(*count, example)?;
        }
        Commands::Convert { output } => {
            info!(
                "Converting JSON from stdin to Parquet files in {}",
                output.display()
            );
            ingestion::process_json_to_parquet(output)?;
        }
    }

    Ok(())
}
