use clap::Parser;
use client::{AnyClient, Client};
use log::warn;

use crate::{
    cli::Opts,
    db::{Database, Location},
    services::blockchain::BlockchainService,
    ui::run,
};

pub mod cli;
pub mod client;
pub mod db;
pub mod services;
pub mod ui;
pub mod utils;

/// Retrieve an initial block from the endpoint so that upon UI startup there's data to render
#[allow(clippy::needless_question_mark)] /* clippy gets this wrong */
async fn populate_db(opts: &Opts, db: &mut Database) -> eyre::Result<()> {
    Ok(db.add_block(
        &AnyClient::new(opts.rpc.clone())
            .await?
            .block(alloy::eips::BlockNumberOrTag::Latest)
            .await?,
    )?)
}

fn main() -> eyre::Result<()> {
    pretty_env_logger::init_timed();
    let opts: Opts = Opts::parse();

    if opts.headless && opts.db.is_none() {
        warn!("Headless mode without specifying an on-disk database. All data will be lost on exit.");
    }

    let mut db: Database = Database::new(match opts.db {
        Some(ref file) => Location::Disk(file.to_path_buf()),
        None => Location::Memory,
    })?;

    if opts.list_block_hashes {
        db.all_block_hashes()?
            .iter()
            .for_each(|hash| println!("{hash}"));
    }

    /* wet the database */
    tokio::task::block_in_place(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { populate_db(&opts, &mut db).await })
    })?;

    let blockchain = BlockchainService::spawn(opts.rpc, db.clone());

    if !opts.headless {
        let terminal = ratatui::init();
        let result = run(terminal, &db);
        ratatui::restore();
        result
    } else {
        let _ = blockchain.join();
        Ok(())
    }
}
