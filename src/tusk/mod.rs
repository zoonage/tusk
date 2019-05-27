/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod filter;
pub mod pipeline;
pub mod wrapper;

use crate::plugins::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub type Interval = u32;

#[derive(Deserialize)]
#[serde(default)]
pub struct Tusk {
    /// The default number of seconds between input plugin gathers.
    input_interval: Interval,
    /// The default number of seconds between output plugin sends.
    output_interval: Interval,
    /// Some common dimensions to apply to every reading
    dimensions: Dimensions,
    /// Some common tags to apply to every reading
    tags: Vec<Tag>,
    /// Loaded input plugins
    #[serde(skip)]
    inputs: HashMap<String, wrapper::Wrapper<InputPlugin>>,
    /// Loaded output plugins
    #[serde(skip)]
    outputs: HashMap<String, wrapper::Wrapper<OutputPlugin>>,
    /// Loaded pipelines
    #[serde(skip)]
    pipelines: HashMap<String, pipeline::Pipeline>,
}

impl Default for Tusk {
    fn default() -> Self {
        Tusk {
            input_interval: 10,
            output_interval: 10,
            ..Default::default()
        }
    }
}

impl Tusk {
    pub fn from_directory(config_dir_path: &Path) -> Result<Self, Error> {
        let config_dir = config_dir_path.to_path_buf();
        let mut master_config_file_buf = config_dir.clone();
        master_config_file_buf.push("tusk.yaml");
        let master_config_file = master_config_file_buf.as_path();
        debug!("Loading master config file {:?}", master_config_file);

        /// Load the agent
        let reader = File::open(&master_config_file_buf)?;
        let agent: Tusk = serde_yaml::from_reader(reader)?;

        let mut inputs_config_dir_buf = config_dir.clone();
        inputs_config_dir_buf.push("inputs");
        let inputs_config_dir = inputs_config_dir_buf.as_path();
        debug!("Loading inputs config dir {:?}", inputs_config_dir);

        let mut outputs_config_dir_buf = config_dir.clone();
        outputs_config_dir_buf.push("outputs");
        let outputs_config_dir = outputs_config_dir_buf.as_path();
        debug!("Loading outputs config dir {:?}", outputs_config_dir);

        let mut pipelines_config_dir_buf = config_dir.clone();
        pipelines_config_dir_buf.push("pipelines");
        let pipelines_config_dir = pipelines_config_dir_buf.as_path();
        debug!("Loading pipelines config dir {:?}", pipelines_config_dir);

        Ok(agent)
    }

    pub fn add_input_plugin(&mut self, name: &str, ip: &wrapper::Wrapper<InputPlugin>) {
        self.inputs.insert(name.to_string(), ip.clone());
    }

    pub fn add_output_plugin(&mut self, name: &str, op: &wrapper::Wrapper<OutputPlugin>) {
        self.outputs.insert(name.to_string(), op.clone());
    }

    pub fn add_pipeline(&mut self, name: &str, pl: &pipeline::Pipeline) {
        self.pipelines.insert(name.to_string(), pl.clone());
    }

    pub fn start(&mut self) -> Result<(), Error> {
        // Start all the outputs first
        Ok(())
    }
}
