use clap::Parser;
use clap_derive::Subcommand;

#[derive(Parser, Debug)]
#[clap(
    name = "Lumi",
    version = "0.0.1",
    author = "Martin Bucinskas <martinb.dev>",
    about = "Lumi General Programming Language VM"
)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Assemble a .iasm or .lumi file
    Assemble {
        /// Path to the .iasm or .lumi file
        #[arg(short, long)]
        input_file: Option<String>,
    },
    /// Opens a REPL console
    Console {
        /// Number of OS threads for the VM to utilize
        #[arg(short, long)]
        threads: Option<usize>,
        /// Enable SSH server
        #[arg(long)]
        enable_ssh: bool,
        /// Port for SSH server
        #[arg(short, long)]
        ssh_port: Option<u16>,
    },
    /// Runs an assembled program on the VM, opens in REPL mode
    Run {
        /// Path to the assembled .bin file to run
        #[arg(short, long)]
        input_file: Option<String>,
        /// Number of OS threads for the VM to utilize
        #[arg(short, long)]
        threads: Option<usize>,
        /// Enable SSH server
        #[arg(long)]
        enable_ssh: bool,
        /// Port for SSH server
        #[arg(short, long)]
        ssh_port: Option<u16>,
    },
    /// Add an SSH key to the cluster
    AddSshKey {
        /// Path to the public key to be used for SSH
        #[arg(short, long)]
        pub_key_file: String,
    },
}
