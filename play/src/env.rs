lazy_static! {
    pub static ref SENTRY_ENABLED: bool = realm::env::bool_with_default("SENTRY_ENABLED", false);
    pub static ref OBSERVER_LOG_PATH: String =
        std::env::var("OBSERVER_LOG_PATH").unwrap_or_else(|_| "/tmp/observer.log".to_string());
}

pub fn check() {
    realm::env::check();
    lazy_static::initialize(&SENTRY_ENABLED);
    lazy_static::initialize(&OBSERVER_LOG_PATH);
}
