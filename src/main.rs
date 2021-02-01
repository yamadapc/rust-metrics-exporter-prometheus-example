use async_std::net::SocketAddr;
use metrics::increment_counter;
use metrics::register_counter;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Building Prometheus metrics recorder");
    PrometheusBuilder::new()
        .listen_address(addr)
        .install()
        .unwrap();

    log::info!("Starting loop");
    register_counter!("test_counter");

    loop {
        log::info!("Increment `test_counter`");
        increment_counter!("test_counter");
        thread::sleep(Duration::from_secs(1));
    }
}
