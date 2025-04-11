use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

pub fn register_subscriber(prefix: &str) -> WorkerGuard {
    let log_file = rolling::daily("../../../devops/develop/loki", prefix);
    let (non_blocking, _guard) = tracing_appender::non_blocking(log_file);

    Registry::default()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    _guard
}
