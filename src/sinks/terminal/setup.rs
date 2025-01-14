use std::{sync::mpsc::Receiver, time::Duration};

use serde_derive::Deserialize;

use crate::framework::{BootstrapResult, Event, SinkConfig};

use super::run::reducer_loop;

const THROTTLE_MIN_SPAN_MILLIS: u64 = 300;

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    throttle_min_span_millis: Option<u64>,
}

impl SinkConfig for Config {
    fn bootstrap(&self, input: Receiver<Event>) -> BootstrapResult {
        let throttle_min_span = Duration::from_millis(
            self.throttle_min_span_millis
                .unwrap_or(THROTTLE_MIN_SPAN_MILLIS),
        );

        let handle = std::thread::spawn(move || {
            reducer_loop(throttle_min_span, input).expect("terminal sink loop failed");
        });

        Ok(handle)
    }
}
