use clap::Parser;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

mod cli;
mod feed;
mod logging;
mod metadata;
mod prelude;

use feed::Feed;
use metadata::{Metadata, MetadataExt};

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

async fn generate_content(args: cli::GenArgs) -> anyhow::Result<()> {
    let meta = Metadata::from_path(&args.in_dir).await?;

    let meta_out_path = args.out_dir.join("metadata.json");
    let buf = serde_json::to_vec(&meta)?;

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(meta_out_path)
        .await?;

    f.write_all(&buf).await?;

    let feed = Feed::from_metadata(&meta);
    feed.write_feeds(&args.out_dir)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init();

    // Ignore .env if fails to load.
    let _ = dotenv::dotenv();

    let args = cli::Args::parse();

    match args.cmd {
        cli::Commands::Generate(args) => generate_content(args).await?,
    }

    Ok(())
}
