use std::{num::NonZeroU64, path::PathBuf, time::Duration};

use iroha_crypto::PrivateKey;
use iroha_data_model::{
    prelude::{MetadataLimits, PeerId, PublicKey},
    transaction::TransactionLimits,
    LengthLimits,
};
use iroha_primitives::addr::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
    pub genesis: genesis::Config,
    pub kura: kura::Config,
    pub network: network::Config,
    pub logger: logger::Config,
    pub sumeragi: sumeragi::Config,
    pub torii: torii::Config,
    pub wsv: wsv::Config,
    pub block_sync: block_sync::Config,
    pub telemetry: telemetry::Config,
    pub queue: queue::Config,
    pub snapshot: Option<snapshot::Config>,
}

pub mod genesis {
    use iroha_crypto::KeyPair;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]

    pub enum Config {
        Submit { path: PathBuf, key_pair: KeyPair },
        // FIXME: name is fine?
        Verify { public_key: PublicKey },
    }
}

pub mod kura {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub init_mode: Mode,
        pub block_store_path: PathBuf,
        pub blocks_per_storage_file: u64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Mode {
        Strict,
        Fast,
    }
}

pub mod logger {
    use iroha_data_model::Level;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub level: Level,
        pub compact_mode: bool,
        pub log_file_path: Option<PathBuf>,
        // got from CLI
        pub terminal_colors: bool,
        // new
        pub telemetry_capacity: u32,
    }
}

pub mod network {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub address: SocketAddr,
    }
}

pub mod sumeragi {
    use iroha_crypto::KeyPair;
    use iroha_primitives::unique_vec::UniqueVec;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        // hidden from user space
        pub peer_id: PeerId,
        // hidden from user space
        pub key_pair: KeyPair,
        pub trusted_peers: UniqueVec<PeerId>,
        pub block_time: Duration,
        pub commit_time: Duration,
        pub gossip_period: Duration,
        pub gossip_batch_amount: u32,
        pub max_transactions_in_block: u32,
    }

    impl Config {
        pub fn consensus_estimation(&self) -> Duration {
            self.block_time + (self.commit_time / 2)
        }
    }
}

pub mod torii {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub api_address: SocketAddr,
        pub telemetry_address: Option<SocketAddr>,
        pub max_transaction_size: ByteSize<u32>,
        pub max_content_len: ByteSize<u32>,
        pub fetch_amount: NonZeroU64,
        pub query_idle_time: Duration,
    }
}

pub mod wsv {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
    pub struct Config {
        pub domain_metadata_limits: MetadataLimits,
        pub account_metadata_limits: MetadataLimits,
        pub asset_definition_metadata_limits: MetadataLimits,
        pub asset_metadata_limits: MetadataLimits,
        pub identifier_length_limits: LengthLimits,
        pub transaction_limits: TransactionLimits,
        pub wasm_runtime: WasmRuntime,
    }

    #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
    pub struct WasmRuntime {
        pub fuel_limit: u64,
        pub max_memory: ByteSize<u32>,
    }
}

pub mod telemetry {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub substrate: Option<Substrate>,
        pub file_output: Option<FileOutput>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Substrate {
        pub name: String,
        pub url: String,
        pub min_retry_period: Duration,
        pub min_retry_delay_exponent: u8,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FileOutput {
        pub file: PathBuf,
    }
}

pub mod block_sync {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub block_batch_amount: u32,
        pub gossip_period: Duration,
    }
}

pub mod queue {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub max_transactions_in_queue: u32,
        pub max_transactions_in_queue_per_user: u32,
        pub transactions_time_to_live: Duration,
        pub future_threshold: Duration,
    }
}

pub mod snapshot {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub create_every: Duration,
        pub directory: PathBuf,
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct ByteSize<T>(pub T);
