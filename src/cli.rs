use std::path::PathBuf;

use clap::Parser;
use url::Url;

/// Minimalist TUI block explorer and chain indexer
#[derive(Clone, Debug, Parser)]
#[clap(version, about, author)]
pub struct Opts {
    #[clap(short, long, default_value = "wss://eth.merkle.io")]
    pub rpc: Url,
    #[clap(short, long)]
    pub db: Option<PathBuf>,
    #[clap(long, action)]
    pub headless: bool,
    #[clap(long, action)]
    pub list_block_hashes: bool,
}
