
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{debug, error, info};

use crate::{b2_token_by_headless_chrome::{b2_token_init, get_b2_token}, config::{UserinfoConfig, get_userinfo}};

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct Lv {
    vip: Vip,
    lv: SubLv,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct Vip {
    name: String,
    lv: String,
    icon: String,
    color: String,
    time: u32,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct SubLv {
    name: String,
    credit: String,
    lv: String,
    icon: String,
    lv_next: String,
    lv_next_name: String,
    lv_next_credit: String,
    lv_ratio: u32,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct Task {
    total: u32,
    finish: u32,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct UserData {
    id: u32,
    sex: u32,
    name: String,
    link: String,
    avatar: String,
    avatar_webp: String,
    desc: String,
    user_title: String,
    verify: String,
    verify_icon: String,
    cover: String,
    cover_webp: String,
    lv: Lv,
    user_code: String,
    is_admin: bool,
    following: u32,
    followers: u32,
    post_count: String,
    comment_count: u32,
    credit: String,
    money: String,
    task: String,
    task_: Task
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct ZwUserInfo {
    write: bool,
    newsflashes: bool,
    infomation: bool,
    create_circle: bool,
    create_topic: bool,
    binding_login: bool,
    user_data: UserData,
    can_img: bool,
    can_ask: bool,
    can_answer: bool,
    can_video: bool,
    can_file: bool,
    carts: u32,
    image_size: String,
    video_size: String,
    file_size: String,
    msg_unread: String,
    dmsg_unread: String
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct GetUserMission {
    mission: Mission,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct Mission {
    date: String,
    credit: String,
    always: String,
    tk: Tk,
    my_credit: String,
    current_user: u32,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct Tk {
    days: u32,
    credit: u32,
    bs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zw {
    b2_token: Option<String>,
    userinfo: UserinfoConfig,
    status: bool,
    credit: String,
    user_mission: GetUserMission,
    user_info: ZwUserInfo,
    #[serde(skip)]
    client: Client,
}

impl Zw {
    pub async fn new() -> Self {
        if let Err(e) = b2_token_init().await {
            error!("B2_token init error: {}", e.to_string());
        }
        let userinfo = get_userinfo().await;
        Self {
            b2_token: None,
            userinfo,
            status: false,
            credit: "".to_string(),
            user_mission: GetUserMission::default(),
            user_info: ZwUserInfo::default(),
            client: Client::new(),
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        debug!("Starting sign...");
        loop {
            self.check_status().await?;
            match self.status {
                true => {
                    info!("{} : {} : {}", self.user_mission.mission.date, self.user_mission.mission.credit, self.user_mission.mission.my_credit);
                    break;
                },
                false => {
                    self.check_b2_token().await?;
                },
            }
        }
        Ok(())
    }

    async fn check_b2_token(&mut self) -> anyhow::Result<()> {
        debug!("Checking b2_token...");
        if self.b2_token.is_none() {
            self.get_b2_token().await?;
        }
        self.sign_in().await?;
        Ok(())
    }

    async fn get_b2_token(&mut self) -> anyhow::Result<()> {
        debug!("Getting b2_token...");
        loop {
            let b2_token = get_b2_token().await;
            match b2_token {
                Some(v) => {
                    self.b2_token = Some(v);
                    return Ok(());
                },
                None => {
                    error!("b2_token 为 null,10分钟后重新获取.");
                    sleep(Duration::from_secs(600)).await;
                },
            }
        }
    }

    async fn check_status(&mut self) -> anyhow::Result<()> {
        debug!("Checking status...");
        if self.user_mission.mission.date.contains(chrono::Local::now().format("%Y-%m-%d").to_string().as_str()) {
            self.status = true;
        } else {
            self.status = false;
        }
        Ok(())
    }

    async fn sign_in(&mut self) -> anyhow::Result<()> {
        debug!("Signing in...");
        let res = self.client.post(&self.userinfo.url1.clone())
            .bearer_auth(&self.b2_token.clone().unwrap())
            .send()
            .await;
        match res {
            Ok(res) => {
                let text = res.text().await?;
                debug!("Sign in response: {}", text);
                self.credit = text;
                self.get_user_mission().await?;
                Ok(())
            },
            Err(e) => {
                debug!("Error signing in: {}", e.to_string());
                debug!("get b2_token and try again");
                self.b2_token = None;
                Ok(())
            }
        }
    }

    #[allow(unused)]
    async fn get_user_info(&mut self) -> anyhow::Result<()> {
        debug!("Getting user info...");
        let res = self.client.post(&self.userinfo.url2.clone())
            .bearer_auth(&self.b2_token.clone().unwrap())
            .send()
            .await?;
        self.user_info = res.json().await?;
        Ok(())
    }

    async fn get_user_mission(&mut self) -> anyhow::Result<()> {
        debug!("Getting user mission...");
        let res = self.client.post(&self.userinfo.url3.clone())
            .bearer_auth(&self.b2_token.clone().unwrap())
            .send()
            .await?;

        self.user_mission = match res.json().await {
            Ok(json) => json,
            Err(e) => {
                debug!("Error getting user mission: {}", e);
                GetUserMission::default()
            }
        };
        Ok(())
    }
}


