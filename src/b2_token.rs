use std::sync::{OnceLock};


use thirtyfour::{By, DesiredCapabilities, WebDriver, prelude::ElementQueryable};
use tokio::sync::Mutex;
use tracing::debug;

use crate::config::get_userinfo;


static B2_TOKEN: OnceLock<Mutex<B2Token>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct B2Token {
    login_url: String,
    forum_url: String,
    username: String,
    password: String,
    driver: WebDriver,
}

impl B2Token {
    pub async fn new() -> Self {
        let userinfo = get_userinfo().await;
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new(userinfo.chrome_driver.clone(), caps.clone()).await.unwrap();
        Self {
            login_url : userinfo.login_url.clone(),
            forum_url : userinfo.forum_url.clone(),
            username : userinfo.username.clone(),
            password : userinfo.password.clone(),
            driver: driver,
        }
    }

    pub async fn get_token(&mut self) -> anyhow::Result<Option<String>> {
        let driver = self.driver.clone();

        driver.goto(self.login_url.clone()).await?;
        driver.find(By::Id("phoneInput")).await?.send_keys(self.username.clone()).await?;
        driver.find(By::Id("passwordInput")).await?.send_keys(self.password.clone()).await?;
        driver.find(By::Id("Agreement")).await?.click().await?;
        driver.find(By::Id("LoginBtn")).await?.click().await?;
        
        driver.query(By::XPath("//*[contains(text(), '欢迎进入中望用户中心')]")).first().await?;

        driver.goto(self.forum_url.clone()).await?;
        let login_btn = driver.query(By::XPath("//*[contains(text(), '登录')]")).first().await?;
        driver.execute("arguments[0].click();", vec![login_btn.to_json()?]).await?;
        driver.query(By::XPath("//*[contains(text(), '积分商城')]")).first().await?;
        
        let b2_token = driver.get_named_cookie("b2_token").await?;
        debug!("b2_token: {}", b2_token.value);
        Ok(Some(b2_token.value))
    }
}

pub async fn b2_token_init() -> anyhow::Result<()> {
    let b2_token = B2Token::new().await;
    B2_TOKEN.set(Mutex::new(b2_token)).map_err(|_| anyhow::anyhow!("Failed to set b2_token"))?;
    Ok(())
}

pub async fn get_b2_token() -> Option<String> {
    let mut b2_token = B2_TOKEN.get().unwrap().lock().await;
    b2_token.get_token().await.unwrap()
}