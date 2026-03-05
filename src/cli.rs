use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hasher")]
#[command(version = "0.1.0")]
#[command(about = "Universal file integrity tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Generate {
        file: String,

        #[arg(long, short)]
        algo: Option<String>,

        #[arg(long, short)]
        output: Option<String>,
    },

    Verify {
        file: String,

        #[arg(long)]
        hash: Option<String>,

        #[arg(long)]
        algo: Option<String>,

        #[arg(long)]
        manifest: Option<String>,
    },

    Algorithms,
}
