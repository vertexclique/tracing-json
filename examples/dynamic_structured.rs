use tracing_json::layers::prelude::*;
use tracing::instrument;
use tracing::info;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

use std::sync::atomic::{AtomicU64, Ordering};

#[instrument]
pub fn a_unit_of_work(first_parameter: u64) {
    for i in 0..2 {
        a_sub_unit_of_work(i);
    }
    info!(excited = "true", "At that unit of work");
}

#[instrument]
pub fn a_sub_unit_of_work(sub_parameter: u64) {
    info!("Events have the full context of their parent span!");
}


fn main() {
    let config: &str = r#"
        {
            "fields": [
                {
                    "name": "entry_id",
                    "dtype": {
                      "type": "dynamic",
                      "return_type": "u64"
                    }
                },
                {
                    "name": "app",
                    "dtype": {
                      "type": "constant",
                      "value": "structured"
                    }
                },
                {
                    "name": "level",
                    "dtype": {
                      "type": "level",
                      "value": "WARN"
                    }
                },
                {
                    "name": "message",
                    "dtype": {
                      "type": "message"
                    }
                },
                {
                    "name": "current_ms",
                    "dtype": {
                      "type": "currentmilliseconds"
                    }
                }
            ]
        }
        "#;

    let mut counter = AtomicU64::default();
    let event_counter: FunStub = || {
        // let x = counter.fetch_add(1_u64, Ordering::SeqCst);
        let x = 1234_u64;
        Box::new(x)
    };
    // counter.fetch_add(1, Ordering::Relaxed)


    let mut formatting_layer = Structured::new(config, std::io::stdout).unwrap();
    formatting_layer.functions(
        vec![
            ("entry_id".to_owned(), event_counter)
        ]
    );

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Orphan event without a parent span");
    a_unit_of_work(2);
}
