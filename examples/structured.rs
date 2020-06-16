use tracing::info;
use tracing::instrument;
use tracing_json::layers::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

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
                    "name": "app",
                    "dtype": {
                      "type": "constant",
                      "value": "structured-app"
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
    let formatting_layer = Structured::new(config, std::io::stdout).unwrap();

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Orphan event without a parent span");
    a_unit_of_work(2);
}
