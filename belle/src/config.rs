use clap::CommandFactory;
use clap::Parser;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Cli> = Lazy::new(declare_config);

#[derive(Parser, Debug)]
#[command(name = "belle")]
#[command(version = "0.2.0")]
#[command(author = "gummi")]
#[command(about = "BELLE - The Big Endian, Low Level Emulator", long_about = None)]
pub struct Cli {
    /// Path to input
    #[clap(required = true)]
    pub file: String,

    /// Verbose output
    #[clap(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Display debug messages
    #[clap(short = 'd', long, default_value_t = false)]
    pub debug: bool,

    /// Quiet (do not print errors)
    #[clap(short = 'q', long, default_value_t = false)]
    pub quiet: bool,

    /// Clock cycle delay (milliseconds)
    #[clap(short = 't', long)]
    pub time_delay: Option<u32>,

    /// Print the state of the CPU when it halts
    #[clap(short = 'p', long, default_value_t = false)]
    pub pretty: bool,

    /// Fuzzing mode
    #[clap(short = 'f', long, default_value_t = false)]
    pub fuzz: bool,

    /// Write crash to file
    #[clap(short = 'w', long, default_value_t = false)]
    pub write: bool,
}

pub fn declare_config() -> Cli {
    Cli::try_parse().unwrap_or_else(|_| {
        Cli::command().print_help().unwrap();
        println!();
        Cli {
            file: "".to_string(),
            debug: false,
            verbose: false,
            quiet: false,
            time_delay: None,
            pretty: false,
            fuzz: false,
            write: false,
        }
    })
}
