use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::constants::ethereum;

#[derive(Args)]
pub struct L1FetcherOptions {
    /// The Ethereum JSON-RPC HTTP URL to use.
    #[arg(long)]
    pub http_url: String,
    /// Ethereum block number to start state import from.
    #[arg(short, long, default_value_t = ethereum::GENESIS_BLOCK)]
    pub start_block: u64,
    /// The number of blocks to filter & process in one step over.
    #[arg(short, long, default_value_t = ethereum::BLOCK_STEP)]
    pub block_step: u64,
    /// The number of blocks to process from Ethereum.
    #[arg(long)]
    pub block_count: Option<u64>,
}

#[derive(Subcommand)]
pub enum ReconstructSource {
    /// Fetch data from L1.
    L1 {
        #[command(flatten)]
        l1_fetcher_options: L1FetcherOptions,
    },
    /// Fetch data from a file.
    File {
        /// The path of the file to import state from.
        file: String,
    },
}

#[derive(ValueEnum, Clone)]
pub enum Query {
    /// The latest root hash of the current tree.
    RootHash,
}

#[derive(Subcommand)]
pub enum Command {
    /// Download L2 state from L1 to JSON file.
    #[command(hide = true)]
    Download {
        #[command(flatten)]
        l1_fetcher_options: L1FetcherOptions,
        /// The path of the file to save the state to.
        file: String,
    },

    /// Reconstruct L2 state from a source.
    Reconstruct {
        /// The source to fetch data from.
        #[command(subcommand)]
        source: ReconstructSource,
        /// The path to the storage solution.
        #[arg(short, long, env = "ZK_SYNC_DB_PATH")]
        db_path: Option<String>,
    },

    /// Query the local storage, and optionally, return a JSON-payload of the data.
    Query {
        /// The query to run.
        #[arg(index = 1)]
        query: Query,
        /// If present, print the data in JSON-compliant format.
        #[arg(short, long)]
        json: bool,
        /// The path to the storage solution.
        #[arg(short, long, env = "ZK_SYNC_DB_PATH")]
        db_path: Option<String>,
    },
}

#[derive(Parser)]
#[command(author, version, about = "zkSync state reconstruction tool")]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Command,
}
