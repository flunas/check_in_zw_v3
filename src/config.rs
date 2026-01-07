use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};


static MYCONFIG: OnceLock<Mutex<MyConfig>> = OnceLock::new();

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct MyConfig {
    pub userinfo: UserinfoConfig,
    pub schedule: ScheduleConfig,
    pub log: LogConfig,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct UserinfoConfig {
    pub username: String,
    pub password: String,
    pub url1: String,
    pub url2: String,
    pub url3: String,
    pub login_url: String,
    pub forum_url: String,
    pub chrome_driver: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ScheduleConfig {
    pub cron: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LogConfig {
    pub enable: bool,
    pub level: String,
}

impl MyConfig {
    pub fn new() -> Self {
        let config = config::Config::builder()
            .add_source(config::File::with_name("/app/config/config.toml"))
            .build()
            .unwrap()
            .try_deserialize::<MyConfig>()
            .unwrap();
        config
    }

    pub fn get_userinfo(&self) -> UserinfoConfig {
        self.userinfo.clone()
    }

    pub fn get_schedule(&self) -> ScheduleConfig {
        self.schedule.clone()
    }

    pub fn get_log(&self) -> LogConfig {
        self.log.clone()
    }
}

pub async fn my_config_init() -> anyhow::Result<()> {
    let config = MyConfig::new();
    MYCONFIG.set(Mutex::new(config)).map_err(|_| anyhow::anyhow!("Failed to set config"))?;
    Ok(())
}

pub async fn get_userinfo() -> UserinfoConfig {
    let config = MYCONFIG.get().unwrap().lock().unwrap();
    config.get_userinfo()
}

pub async fn get_schedule() -> ScheduleConfig {
    let config = MYCONFIG.get().unwrap().lock().unwrap();
    config.get_schedule()
}

pub async fn get_log() -> LogConfig {
    let config = MYCONFIG.get().unwrap().lock().unwrap();
    config.get_log()
}