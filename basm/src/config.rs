use clap::Parser;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Args> = Lazy::new(declare_config);

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "basm")]
#[command(version = "0.5.0")]
#[command(author = "gummi")]
#[command(about = "The assembler for BELLE", long_about = None)]
pub struct Args {
    /// Output binary name
    #[clap(short = 'o', long)]
    pub output: Option<String>,

    /// Source code (required unless in REPL mode)
    #[clap(required_unless_present = "repl")]
    pub source: Option<String>,

    /// Verbose output
    #[clap(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Disable version and start information in binary
    /// (may cause unexpected behavior in emulator and disassembler)
    #[clap(short = 't', long, default_value_t = false, verbatim_doc_comment)]
    pub thin: bool,

    /// REPL mode
    #[clap(short = 'r', long, default_value_t = false, verbatim_doc_comment)]
    pub repl: bool,
}

pub fn declare_config() -> Args {
    let cli = Args::parse();

    let output = cli.output.unwrap_or_else(|| "a.out".to_string());

    Args {
        source: cli.source,
        output: Some(output),
        verbose: cli.verbose,
        thin: cli.thin,
        repl: cli.repl,
    }
}
