/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use failure::Fail;

pub mod inputs;
pub mod outputs;
pub mod processors;

pub type PluginResult<T> = Result<T, PluginError>;

#[derive(Debug, Fail)]
pub enum PluginError {
    #[fail(display = "The wrong plugin configuration has been provided")]
    InvalidPluginConfiguration,
    #[fail(display = "Unable to gather reading: {}", reason)]
    GatherError { reason: String },
    #[fail(display = "Formatter does not support deserialisation")]
    FormatterDeserializeNotImplemented,
}

pub mod prelude {
    pub use std::io::{BufReader, BufWriter, Read, Write};
    pub use std::thread;
    pub use std::time::Duration;

    pub use bincode;
    pub use crossbeam::channel::bounded as bounded_channel;
    pub use crossbeam::channel::{select, Receiver, Sender};
    pub use failure::Error;
    pub use log::{debug, error, info, trace, warn};
    pub use serde::{Deserialize, Serialize};
    pub use serde_json;
    pub use serde_yaml;

    pub use super::inputs::{Input, InputFormat, InputPlugin};
    pub use super::outputs::{Output, OutputFormat, OutputPlugin};
    pub use super::processors::{Processor, ProcessorPlugin};
    pub use super::{PluginError, PluginResult};

    pub use crate::common::*;
}
