use structopt::StructOpt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

mod cli;
mod feed;
mod metadata;
mod prelude;

use feed::Feed;
use metadata::{Metadata, MetadataExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::from_args();

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
    feed.write_feeds(args.out_dir)?;

    Ok(())
}
