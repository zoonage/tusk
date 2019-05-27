/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::plugins::prelude::select;
use crate::plugins::prelude::*;

pub struct AddTags {
    tags: Vec<Tag>,
}

impl Processor for AddTags {
    fn process(&self, input: Receiver<Reading>, output: Sender<Reading>) -> PluginResult<()> {
        let tags = self.tags.clone();
        thread::spawn(move || loop {
            select! {
                recv(input) -> reading => {
                    match reading {
                        Ok(mut r) => {
                            for tag in &tags {
                                r.add_tag(&tag);
                            }
                            match output.send(r) {
                                Ok(_) => {},
                                Err(e) => error!("Unable to send reading: {}", e),
                            }
                        },
                        Err(e) => error!("Unable to receive reading: {}", e),
                    }
                }
            }
        });
        Ok(())
    }
    fn name(&self) -> String {
        "add_tag".to_string()
    }
}

#[cfg(test)]
mod tests {
    
    
}
