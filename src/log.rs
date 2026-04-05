use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{filter::Targets, fmt::{self, time::LocalTime}, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::get_log;

pub struct MyLog {
    log_enable: bool,
    log_level: String,
}

impl MyLog {
    pub fn new() -> Self {
        let log_enable = false;
        let log_level = "info".to_string();
        Self {
            log_enable,
            log_level,
        }
    }

    pub async fn run(&mut self) {
        let log_config = get_log().await;
        self.log_enable = log_config.enable;
        self.log_level = log_config.level;
        if self.log_enable {
            let level = match self.log_level.to_lowercase().as_str() {
                "trace" => Level::TRACE,
                "debug" => Level::DEBUG,
                "info" => Level::INFO,
                "warn" | "warning" => Level::WARN,
                "error" => Level::ERROR,
                _ => Level::INFO,
            };
            let filter = Targets::new()
                .with_default(LevelFilter::OFF)
                .with_target("check_in_zw_v3", level);
            
            tracing_subscriber::registry()
                .with(fmt::layer().with_timer(LocalTime::rfc_3339()))
                .with(filter)
                .init();
        }
    }
}