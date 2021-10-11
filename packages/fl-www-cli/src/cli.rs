use std::path::PathBuf;

use structopt::clap::AppSettings::{ColorAuto, ColoredHelp};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(author, about, global_setting(ColoredHelp), global_setting(ColorAuto))]
pub(crate) struct Args {
    /// input directory
    #[structopt(short, long)]
    pub in_dir: PathBuf,
    /// output directory
    #[structopt(short, long)]
    pub out_dir: PathBuf,
}
