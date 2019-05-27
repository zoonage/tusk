/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use rand;

use crate::plugins::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Random {
    min: f64,
    max: f64,
}

impl Input for Random {
    fn gather(&self) -> PluginResult<Vec<Reading>> {
        let mut reading = Reading::new();
        reading.add_datapoint(
            "value",
            rand::random::<f64>() * (self.max - self.min) + self.min,
        );
        Ok(vec![reading])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_random(min: f64, max: f64) {
        let r = Random {
            min: min.clone(),
            max: max.clone(),
        };
        for _ in 1..100 {
            let n = r.gather().unwrap()[0]
                .datapoints
                .get("value")
                .unwrap()
                .clone();
            assert!(n >= min);
            assert!(n <= max);
        }
    }

    #[test]
    fn test_random_int() {
        test_random(1.0, 2.0);
    }

    #[test]
    fn test_random_float() {
        test_random(100.12314145, 456735678.1230);
    }
}
