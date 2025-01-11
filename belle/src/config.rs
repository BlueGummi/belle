#[allow(unused_imports)] // tests
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
    /// Path to ROM
    #[clap(required = true)]
    pub rom: String,

    /// Verbose output
    #[clap(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Enter debugger
    #[clap(short = 'd', long, default_value_t = false)]
    pub debug: bool,

    /// Do not print errors
    #[clap(short = 'q', long, default_value_t = false)]
    pub quiet: bool,

    /// Clock delay (milliseconds)
    #[clap(short = 't', long)]
    pub time_delay: Option<u32>,

    /// Print the state of the CPU when it halts
    #[clap(short = 'p', long, default_value_t = false)]
    pub pretty: bool,

    /// Write crash to file
    #[clap(short = 'w', long, default_value_t = false)]
    pub write: bool,

    /// Write crash to file
    #[clap(short = 'n', long, default_value_t = false)]
    pub no_print_memory: bool,
}
pub fn declare_config() -> Cli {
    Cli::try_parse().unwrap_or_else(|_| {
        #[cfg(not(test))]
        #[cfg(not(fuzzing))]
        {
            Cli::command().print_help().unwrap();
            std::process::exit(0);
        }

        #[allow(unreachable_code)]
        #[cfg(not(fuzzing))]
        {
            Cli {
                rom: "".to_string(),
                debug: false,
                verbose: false,
                quiet: false,
                time_delay: None,
                pretty: false,
                write: false,
                no_print_memory: false,
            }
        }
        #[cfg(fuzzing)]
        {
            return Cli {
                rom: "".to_string(),
                debug: false,
                verbose: false,
                quiet: false,
                time_delay: None,
                pretty: false,
                write: false,
                no_print_memory: true,
            };
        }
    })
}
