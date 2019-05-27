/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::plugins::prelude::*;

use std::io;

#[derive(Clone, Deserialize)]
pub struct Stdout {
    format: OutputFormat,
}

impl Output for Stdout {
    fn send(&self, readings: &Vec<Reading>) -> PluginResult<Vec<Reading>> {
        self.format
            .to_writer(BufWriter::new(io::stdout()), &readings);
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    
    
}
