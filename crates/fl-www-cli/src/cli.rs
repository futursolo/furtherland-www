use std::path::PathBuf;

use clap::{Args as CmdArgs, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(author, about)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub(crate) enum Commands {
    /// Generates Content
    Generate(GenArgs),

    /// Starts the Backend and serve API endpoint at specified address
    Serve(ServeArgs),
}

#[derive(CmdArgs, Clone, Debug)]
pub(crate) struct GenArgs {
    /// input directory
    #[clap(short, long)]
    pub in_dir: PathBuf,
    /// output directory
    #[clap(short, long)]
    pub out_dir: PathBuf,
}

#[derive(CmdArgs, Clone, Debug)]
pub(crate) struct ServeArgs {
    /// the address to listen
    #[clap(short, long)]
    pub addr: String,
}
