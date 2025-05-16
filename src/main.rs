mod csv_validator;
mod csv_value;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    csv: String,

    #[arg(short, long)]
    schema: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    csv_validator::validate(&args.schema, &args.csv)?;

    Ok(())
}
