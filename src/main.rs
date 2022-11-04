use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry,
};
use tracing_tree::HierarchicalLayer;

#[tracing::instrument]
fn func1(x: u64, y: u64) {
    for _ in 0..y {
        func2(x);
    }
}

#[tracing::instrument]
fn func2(x: u64) {
    std::thread::sleep(std::time::Duration::from_millis(x))
}

#[tokio::main]
async fn main() {
    let tracer = opentelemetry_jaeger::new_collector_pipeline()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .with_endpoint("http://localhost:14268/api/traces")
        .with_reqwest()
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(
            HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .with(telemetry)
        .init();
    loop {
        println!("Starting");
        func1(100, 10);
        println!("Done");
    }
}
