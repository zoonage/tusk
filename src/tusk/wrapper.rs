use crate::plugins::prelude::*;


use super::filter;
use super::Interval;

/// A wrapper around a plugin to provide some common
/// functionality
#[derive(Deserialize, Clone)]
pub struct Wrapper<P> {
    wrapped: P,
    #[serde(default)]
    interval: Option<Interval>,
    #[serde(default)]
    filter: Option<filter::Filter>,
    #[serde(default)]
    tags: Vec<Tag>,
    #[serde(default)]
    dimensions: Dimensions,
}

impl Wrapper<InputPlugin> {
    pub fn start(&self, default_interval: Interval) {
        let _interval = self.interval.unwrap_or(default_interval);
    }
}

impl Wrapper<OutputPlugin> {
    pub fn start(&self, default_interval: Interval) {
        let _interval = self.interval.unwrap_or(default_interval);
    }
}
