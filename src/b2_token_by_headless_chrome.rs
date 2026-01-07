use std::sync::{OnceLock};

use headless_chrome::{Browser, LaunchOptions};
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
}

impl B2Token {
    pub async fn new() -> Self {
        let userinfo = get_userinfo().await;
        Self {
            login_url : userinfo.login_url.clone(),
            forum_url : userinfo.forum_url.clone(),
            username : userinfo.username.clone(),
            password : userinfo.password.clone(),
        }
    }

    pub async fn get_token(&mut self) -> anyhow::Result<Option<String>> {
        // let mut options = LaunchOptions::default();
        // options.headless = true;
        // options.window_size = Some((2560, 1440));
        let options = LaunchOptions::default_builder()
            .sandbox(false)
            .window_size(Some((2560, 1440)))
            .build()?;
        let browser = Browser::new(options)?;
        let  tab = browser.new_tab()?;

        tab.navigate_to(&self.login_url)?.wait_until_navigated()?;
        tab.find_element("input#phoneInput")?.type_into(&self.username)?;
        tab.find_element("input#passwordInput")?.type_into(&self.password)?;
        tab.find_element("input#Agreement")?.click()?;
        tab.find_element("a#LoginBtn")?.click()?;
        tab.wait_until_navigated()?;

        tab.wait_for_xpath("//*[contains(text(), '欢迎进入中望用户中心')]")?;

        tab.navigate_to(&self.forum_url)?.wait_until_navigated()?;
        let js_code = "document.querySelector('button.mobile-hidden').click()";
        tab.evaluate(js_code, false)?;
        tab.wait_until_navigated()?;

        tab.wait_for_xpath("//*[contains(text(), '积分商城')]")?;


        let cookie = tab.get_cookies()?;
        let mut b2_token = None;
        match cookie.iter().find(|k| k.name == "b2_token") {
            Some(k) => b2_token = Some(k.value.clone()),
            None => println!("b2_token cookie found"),
        }
        debug!("b2_token: {:?}", b2_token);
        Ok(b2_token)
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