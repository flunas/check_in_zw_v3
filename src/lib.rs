
use chrono::Offset;
use tokio_cron_scheduler::{Job, JobScheduler};

use tracing::debug;

use crate::{b2_token_by_headless_chrome::{b2_token_init, get_b2_token}, config::get_schedule};


mod sign;
mod config;
mod log;
mod b2_token;
mod b2_token_by_headless_chrome;


pub struct App {
    scheduler: JobScheduler,
}

impl App {
    pub async fn new() -> Self {
        App {
            scheduler: JobScheduler::new().await.unwrap(),
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        // 读取配置
        config::my_config_init().await?;
        // 初始化日志
        log::MyLog::new().run().await;
        // 初始化b2_token
        b2_token_init().await?;

        // 签到-api
        let zw = sign::Zw::new(get_b2_token().await).await;

        let cron = get_schedule().await.cron;
        let cron = local_cron_to_utc(cron.clone()).await;
        let job = Job::new_async(   cron.clone(), move |_uuid,_i| {
            let mut zw = zw.clone();
            Box::pin(async move {
                // 等待执行的异步操作
                debug!("date: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
                zw.run().await.unwrap();
            })
        })?;
        self.scheduler.add(job).await?;
        self.scheduler.start().await?;

        // driver.screenshot(Path::new("./1.png")).await?;
        // driver.quit().await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        self.scheduler.shutdown().await?;
        Ok(())
    }
}


async fn local_cron_to_utc(local_cron: String) -> String {
    let local_time = chrono::Local::now();
    let timezone_offset_hours = local_time.offset().fix().local_minus_utc() as f64 / 3600.0;
    let parts: Vec<&str> = local_cron.split_whitespace().collect();
    
    if parts.len() != 6 {
        return local_cron.to_string(); // 格式错误，返回原样
    }
    
    // 只转换小时部分
    if let Ok(hour) = parts[2].parse::<i32>() {
        let utc_hour = (hour - timezone_offset_hours as i32).rem_euclid(24);
        format!("{} {} {} {} {} {}", 
            parts[0], parts[1], utc_hour, parts[3], parts[4], parts[5])
    } else {
        local_cron.to_string() // 转换失败，返回原样
    }
}
