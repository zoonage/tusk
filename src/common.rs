/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time;

/// The types of value that can be understood.
pub type MeasurementName = String;

/// The types of value that can be understood.
pub type Value = f64;

/// The format for the timestamp as the number of nanoseconds since the epoch time.
pub type TimeStamp = u128;

/// The format for the dimensions of a reading
pub type Dimensions = HashMap<String, String>;

/// The format for the tags of a reading
pub type Tag = String;

/// A combination of data points and metadata about the source.
#[derive(Debug, Serialize, Deserialize)]
pub struct Reading {
    pub datapoints: HashMap<MeasurementName, Value>,
    pub timestamp: TimeStamp,
    pub dimensions: Dimensions,
    pub tags: Vec<Tag>,
}

impl Reading {
    pub fn new() -> Self {
        Reading {
            datapoints: HashMap::new(),
            timestamp: time::SystemTime::now()
                .duration_since(time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            dimensions: HashMap::new(),
            tags: Vec::new(),
        }
    }

    /// Add and overwrite a dimension in a metric
    pub fn add_dimension(&mut self, key: &str, value: &str) -> &mut Self {
        self.dimensions.insert(key.to_string(), value.to_string());
        self
    }

    /// Add a tag to a metric if it doesn't exist
    pub fn add_tag(&mut self, tag: &str) -> &mut Self {
        let t = tag.to_string();
        if !self.tags.contains(&t) {
            self.tags.push(t);
        }
        self
    }

    /// Add and overwrite a datapoint in a metric
    pub fn add_datapoint(&mut self, name: &str, value: Value) -> &mut Self {
        self.datapoints.insert(name.to_string(), value);
        self
    }
}
