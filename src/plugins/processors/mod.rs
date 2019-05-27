/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod add_tag;

use crate::plugins::prelude::*;

pub enum ProcessorPlugin {
    AddTags(add_tag::AddTags),
}

pub trait Processor {
    /// This is used to set up the processor plugin which should be run in a
    /// thread after exiting OK.
    fn process(&self, input: Receiver<Reading>, output: Sender<Reading>) -> PluginResult<()>;
    /// This returns the name of the plugin.
    fn name(&self) -> String;
}
