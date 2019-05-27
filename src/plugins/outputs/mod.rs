/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod stdout;

use crate::plugins::prelude::*;

pub trait Output {
    /// This is used to set up the output plugin.
    /// By default no initialisation is done.
    fn initialise(&mut self) -> PluginResult<()> {
        Ok(())
    }
    /// This gathers the metrics from the output plugin. and returns the metrics
    /// it failed to write.
    fn send(&self, readings: &Vec<Reading>) -> PluginResult<Vec<Reading>>;
}

#[derive(Deserialize, Clone)]
pub enum OutputPlugin {
    Stdout(stdout::Stdout),
}

#[derive(Clone, Deserialize)]
pub enum OutputFormat {
    JSON,
    YAML,
    Bincode,
    // Influx,
    // Prometheus,
    // Metrics2_0,
    // Statsd,
    // DogStatsd,
}

impl OutputFormat {
    /// Serializes readings into a buffered writer.
    /// and returns a list of readings that failed to send
    pub fn to_writer<O: Write>(
        &self,
        output: BufWriter<O>,
        readings: &Vec<Reading>,
    ) -> Result<(), Error> {
        match self {
            OutputFormat::JSON => serde_json::to_writer(output, &readings)?,
            OutputFormat::YAML => serde_yaml::to_writer(output, &readings)?,
            OutputFormat::Bincode => bincode::serialize_into(output, &readings)?,
            _ => unimplemented!(),
        }
        Ok(())
    }
}
