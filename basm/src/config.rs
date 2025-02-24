use clap::Parser;
pub use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Args> = Lazy::new(declare_config);

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "basm")]
#[command(version = "0.5.0")]
#[command(author = "gummi")]
#[command(about = "The assembler for BELLE", long_about = None)]
pub struct Args {
    /// Binary name
    #[clap(short = 'o', long)]
    pub binary: Option<String>,

    /// Source code
    #[clap(required = true)]
    pub source: String,

    /// Verbose output
    #[clap(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

pub fn declare_config() -> Args {
    let cli = Args::parse();

    let binary = cli.binary.unwrap_or_else(|| "a.out".to_string());

    Args {
        source: cli.source,
        binary: Some(binary),
        verbose: cli.verbose,
    }
}
