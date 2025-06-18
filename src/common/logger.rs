use std::sync::Once;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

static INIT_LOG: Once = Once::new();

/// Initializes the global tracing subscriber with a default log level and formatting.
///
/// This function sets up a tracing subscriber with the following configuration:
/// - Log level is set to "info" by default, which can be overridden by environment variables
/// - Disables log target display
/// - Enables thread IDs and thread names in log output
///
/// # Panics
/// Panics if setting the global default subscriber fails
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true);

    INIT_LOG.call_once(|| {
        let subscriber = Registry::default().with(filter).with(fmt_layer);
        let _ = tracing::subscriber::set_global_default(subscriber);
    });
}
