//! Verification report generator binary

use amalgam_verification::Validator;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "verification-report")]
#[command(about = "Generate validation reports for Amalgam-generated Nickel code")]
struct Args {
    /// Path to generated Nickel files
    #[arg(short, long, default_value = "tests/fixtures/generated")]
    path: PathBuf,

    /// Output report file (markdown)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Skip Nickel type checking
    #[arg(long)]
    skip_typecheck: bool,

    /// Skip schema validation
    #[arg(long)]
    skip_schema: bool,

    /// Output as JSON instead of markdown
    #[arg(long)]
    json: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut validator = Validator::new(&args.path);

    if args.skip_typecheck {
        validator = validator.without_typecheck();
    }

    if args.skip_schema {
        validator = validator.without_schema_validation();
    }

    let report = validator.validate_all()?;

    let output = if args.json {
        report.to_json()?
    } else {
        report.to_markdown()
    };

    if let Some(output_path) = args.output {
        std::fs::write(&output_path, &output)?;
        println!("Report written to: {}", output_path.display());
    } else {
        println!("{}", output);
    }

    if !report.summary.success {
        std::process::exit(1);
    }

    Ok(())
}
