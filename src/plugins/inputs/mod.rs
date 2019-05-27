/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod random;

use crate::plugins::prelude::*;

pub trait Input {
    /// Initialises the input plugin if there is any setup to do.
    fn initialise(&mut self) -> PluginResult<()> {
        Ok(())
    }
    /// Causes the input plugin to destroy itself/
    fn destroy(&mut self) {}
    /// This gathers the metrics from the input plugin.
    fn gather(&self) -> PluginResult<Vec<Reading>>;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum InputPlugin {
    Random(random::Random),
}

/// Convinience wrapper for the input plugins enum.
impl Input for InputPlugin {
    fn initialise(&mut self) -> PluginResult<()> {
        match self {
            InputPlugin::Random(p) => p.initialise(),
        }
    }
    fn destroy(&mut self) {
        match self {
            InputPlugin::Random(p) => p.destroy(),
        }
    }
    fn gather(&self) -> PluginResult<Vec<Reading>> {
        match self {
            InputPlugin::Random(p) => p.gather(),
        }
    }
}

pub enum InputFormat {
    JSON,
    YAML,
    Bincode,
    // Influx,
    // Prometheus,
    // Metrics2_0,
    // Statsd,
    // DogStatsd,
}

impl InputFormat {
    /// Deserializes readings from a buffered reader.
    pub fn from_reader<I: Read>(&self, input: BufReader<I>) -> Result<Vec<Reading>, Error> {
        let readings = match self {
            InputFormat::JSON => serde_json::from_reader(input)?,
            InputFormat::YAML => serde_yaml::from_reader(input)?,
            InputFormat::Bincode => bincode::deserialize_from(input)?,
            _ => unimplemented!(),
        };
        Ok(readings)
    }
}
