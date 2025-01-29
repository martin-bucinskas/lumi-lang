use clap::Parser;
use clap_derive::Subcommand;

#[derive(Parser, Debug)]
#[clap(
    name = "Lumi v2",
    version = "2.0.1",
    author = "Martin Bucinskas <martinb.dev>",
    about = "Lumi GPL VM"
)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Assemble a .lmi file
    Assemble {
        /// Path to the .lmi file
        #[arg(short, long)]
        input_file: Option<String>,
    },
    /// Run an assembled program on the VM
    Run {
        /// Path to the assembled file to run
        #[arg(short, long)]
        input_file: String,
    },
    /// Open a REPL console
    Console {
        
    }
}
