/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Tusk is a monitoring agent/server.

use clap;
use env_logger;
use failure::Error;
use log::{info};

/// Contains common definitions used throughout Tusk.
pub mod common;
/// Contains the plugins for Tusk.
///
/// Plugins are divided into categories;
///
/// * Inputs - These plugins gather readings.
/// * Outputs - These plugins put readings somewhere.
/// * Processors - These plugins modify readings as part of a stream.
pub mod plugins;
/// Contains the agent.
pub mod tusk;

use std::env;
use std::path::Path;

/// Starts the Tusk agent.
pub fn main() -> Result<(), Error> {
    env_logger::init();
    let matches = clap::App::new("tusk")
        .author("Aaron George")
        .version(clap::crate_version!())
        .about("The Tusk time series agent and server")
        .arg(
            clap::Arg::with_name("config_dir")
                .short("d")
                .long("config-directory")
                .help("The configuration directory to use"),
        )
        .get_matches();

    // TODO: Support non-linux systems for default paths.
    let config_dir = Path::new(
        matches
            .value_of("config_dir")
            .unwrap_or(&env::var("TUSK_CONFIG_DIR").unwrap_or("/etc/tusk".to_string())),
    )
    .to_path_buf();
    info!("Loading configuration from {:?}", config_dir);

    let _agent = tusk::Tusk::from_directory(&config_dir)?;
    Ok(())
}
