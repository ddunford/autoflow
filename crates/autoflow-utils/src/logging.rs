// Logging utilities

pub fn init_logging(level: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(level)
        .with_target(false)
        .init();
}
