use tracing_subscriber::{fmt, EnvFilter};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;

pub fn init_tracing() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

pub fn init_metrics(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let builder = PrometheusBuilder::new().with_http_listener(addr);
    builder.install()?;
    Ok(())
}