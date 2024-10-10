use helpers::MockWriter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();
    let writer2 = writer.clone();
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap()
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(move || writer.clone())
                .with_span_events(FmtSpan::NEW | FmtSpan::EXIT)
                .json()
                .flatten_event(true)
        )
        .init();
    writer2
}
