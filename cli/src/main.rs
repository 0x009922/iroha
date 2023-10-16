//! Iroha peer command-line interface.
use std::{env, path::PathBuf};

use clap::Parser;
use color_eyre::eyre::WrapErr as _;
use iroha::Iroha;
use iroha_genesis::{GenesisNetwork, RawGenesisBlock};

const DEFAULT_CONFIG_FILE: &'_ str = "config.toml";

#[derive(Debug, Parser)]
#[command(version, about)]
struct Arguments {
    /// Path to the configuration file.
    #[arg(
        short,
        long,
        default_value_t = "PathBuf::from(DEFAULT_CONFIG_FILE)",
        env = "IROHA_CONFIG"
    )]
    pub config: PathBuf,
    /// Flag to enable tracing of configuration resolution
    #[arg(long, env = "IROHA_TRACE_CONFIG")]
    pub trace_config: bool,
    #[arg(short = "s", long, env = "IROHA_SUBMIT_GENESIS")]
    pub submit_genesis: bool,
    #[arg(
        long,
        env = "TERMINAL_COLORS",
        default_value_t = "is_color_supported()"
    )]
    pub terminal_colors: bool,
}

fn is_color_supported() -> bool {
    supports_color::on(supports_color::Stream::Stdout).is_some()
}

#[tokio::main]
/// To make `Iroha` peer work all actors should be started first.
/// After that moment it you can start it with listening to torii events.
///
/// # Side effect
/// - Prints welcome message in the log
///
/// # Errors
/// - Parsing CLI arguments (not an error actually, process simply exits)
/// - Reading & parsing configuration (from file and ENV)
/// - Reading & parsing genesis (if specified in the configuration)
/// - Telemetry setup
/// - Initialization of [`Iroha`]
async fn main() -> Result<(), color_eyre::Report> {
    let args = Arguments::parse();

    if args.terminal_colors {
        if is_color_supported() {
            color_eyre::install()?;
        } else {
            todo!("print a warning")
        }
    }

    let config = iroha::config::load(args)?;

    let telemetry = iroha_logger::init(&config.logger)?;

    iroha_logger::info!(
        version = %env!("CARGO_PKG_VERSION"), git_commit_sha = env!("VERGEN_GIT_SHA"),
        "Hyperledgerいろは2にようこそ！(translation) Welcome to Hyperledger Iroha!"
    );

    // TODO: refactor with parse-not-validate, move to `config::load`
    // assert!(args.submit_genesis || config.sumeragi.trusted_peers.peers.len() > 1,
    //     "Only peer in network, yet required to receive genesis topology. This is a configuration error."
    // );

    let genesis = if let iroha_config2::genesis::Config::Submit { path, key_pair } = &config.genesis
    {
        // TODO: handle different extensions (?) but the extension should be clearly specified...
        let network = RawGenesisBlock::from_path(path.as_ref())
            .and_then(|block| GenesisNetwork::from_configuration(block, key_pair.clone()))
            .wrap_err("Failed to initialize genesis.")?;

        Some(network)
    } else {
        None
    };

    Iroha::with_genesis(genesis, config, telemetry)
        .await?
        .start()
        .await?;
    Ok(())
}

// FIXME: should it be used?
// fn print_version() {
//     println!(
//         "{} {} (git hash {}) \n {}: {}",
//         "Hyperledger Iroha".style(styling.positive),
//         env!("CARGO_PKG_VERSION").style(styling.highlight),
//         env!("VERGEN_GIT_SHA"),
//         "cargo features".style(styling.highlight),
//         env!("VERGEN_CARGO_FEATURES")
//     );
// }
