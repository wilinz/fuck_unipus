use base64::Engine;
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};

use crate::core::html_parser;
use crate::error::unipus::UnipusError;
use crate::model::captcha_response::CaptchaResponse;
use crate::model::class_block::ClassBlock;
use crate::model::sso_login_response::SsoLoginResponse;
use crate::utils::input::input_trim;
use base64::engine::general_purpose::STANDARD;
use reqwest_cookie_store::CookieStoreMutex;
use std::collections::HashMap;
use std::fmt::format;
use std::io::BufWriter;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use scraper::{Html, Selector};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use url::Url;

pub struct Unipus {
    client: Client,
    cookie_store: Arc<CookieStoreMutex>,
    cookie_path: String,
}

impl Unipus {
    pub fn new(username: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36".parse().unwrap());
        let cookie_path = format!("cookies/cookies-{}.jsonl", username);
        let path = Path::new(&cookie_path);

        let cookie_store = if path.exists() {
            match std::fs::File::open(&cookie_path) {
                Ok(file) => {
                    let reader = std::io::BufReader::new(file);
                    reqwest_cookie_store::CookieStore::load_all(reader, |string| {
                        let cookie: cookie_store::Cookie = serde_json::from_str(string)?;
                        Ok::<_, cookie_store::Error>(cookie)
                    }).unwrap_or_else(|err| {
                        eprintln!("警告: 加载 cookie 失败，使用默认值: {}", err);
                        reqwest_cookie_store::CookieStore::default()
                    })
                }
                Err(_) => {
                    eprintln!("警告: 打开 cookie 文件失败，使用默认值");
                    reqwest_cookie_store::CookieStore::default()
                }
            }
        } else {
            if let Some(parent) = path.parent() {
                if let Err(err) = std::fs::create_dir_all(parent) {
                    eprintln!("警告: 创建 cookie 文件夹失败: {}", err);
                }
            }

            if let Err(err) = std::fs::File::create(&cookie_path) {
                eprintln!("警告: 创建 cookie 文件失败: {}", err);
            }
            reqwest_cookie_store::CookieStore::default()
        };


        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);

        let client = Client::builder()
            .default_headers(headers)
            .cookie_provider(std::sync::Arc::clone(&cookie_store))
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        Unipus { client, cookie_store, cookie_path }
    }

    pub async fn get_home_page_and_check_login(&self) -> Result<(String, bool), UnipusError> {
        let response = self.client.get("https://u.unipus.cn/user/student").send().await?;
        let text = response.text().await?;
        let is_authorized= text.contains("我的班课");
        Ok((text, is_authorized))
    }

    pub fn extract_name_form_home_page(&self, html: &str) -> Option<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("div.content_left_top_info_welcome label").ok()?;

        // 获取第一个匹配的元素，并返回其文本内容
        document
            .select(&selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_string())
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
        captcha: Option<&str>,
        encode_captha: Option<&str>,
    ) -> Result<Option<SsoLoginResponse>, UnipusError> {
        self.login_internal(username, password, captcha, encode_captha)
            .await
    }

    fn login_internal<'a>(
        &'a self,
        username: &'a str,
        password: &'a str,
        captcha: Option<&'a str>,
        encode_captha: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Result<Option<SsoLoginResponse>, UnipusError>> + Send + 'a>> {
        Box::pin(async move {

            let _ = self.client.get("https://u.unipus.cn/t?p=https://sso.unipus.cn/sso/login?service=https%3A%2F%2Fu.unipus.cn%2Fuser%2Fcomm%2Flogin%3Fschool_id%3D")
                .send().await?;

            let _ = self
                .client
                .get("https://sso.unipus.cn/sso/3.0/sso/server_time")
                .send()
                .await?;

            let url = "https://sso.unipus.cn/sso/0.1/sso/login";
            let payload = HashMap::from([
                ("service", "https://u.unipus.cn/user/comm/login?school_id="),
                ("username", username),
                ("password", password),
                ("captcha", captcha.unwrap_or("")),
                ("rememberMe", "on"),
                ("captchaCode", captcha.unwrap_or("")),
                ("encodeCaptha", encode_captha.unwrap_or("")),
            ]);

            let response = self.client.post(url).json(&payload).send().await?;
            let data: SsoLoginResponse = response.json().await?;
            // println!("响应0：{:?}", data);

            if data.code == "1506" {
                let captcha_response = self.get_captcha().await?;

                let image_data_url = format!("data:image/png;base64,{}", captcha_response.rs.image);
                println!(
                    "---------验证码已经复制到剪切板，请粘贴验证码内容到浏览器地址栏查看-----------"
                );
                println!("{}", image_data_url);
                println!(
                    "---------------------请粘贴验证码内容到浏览器查看--------------------------"
                );

                let image_base64 = &captcha_response.rs.image;
                let image_data = STANDARD.decode(image_base64).unwrap();
                let captcha_tmp_path = Path::new("tmp/captcha.png");
                std::fs::create_dir_all(captcha_tmp_path.parent().unwrap()).unwrap();
                let mut file = File::create("tmp/captcha.png").await.unwrap();
                file.write_all(&image_data).await.unwrap();

                println!(
                    "---------查看 tmp/captcha.png，或者粘贴验证码 data-url 到浏览器 -----------"
                );

                let captcha = input_trim("请输入验证码：");

                return self
                    .login_internal(
                        username,
                        password,
                        Some(&captcha),
                        Some(&captcha_response.rs.encode_captcha),
                    )
                    .await;
            }else {
                let ticket =  data.rs.as_ref().unwrap().service_ticket.as_str();
                let success = self.login_use_ticket(ticket).await?;
                if success {
                    Ok(None)
                }else {
                    Err(UnipusError::new("ticket登录失败"))
                }
            }
        })
    }

    pub async fn login_use_ticket(&self, ticket: &str) -> Result<bool, UnipusError> {
        let mut url = Url::parse("https://u.unipus.cn/user/comm/login").unwrap();

        url.query_pairs_mut()
            .append_pair("school_id", "");

        if !ticket.is_empty() {
            url.query_pairs_mut().append_pair("ticket", ticket);
        }

        let response = self
            .client
            .get(url)
            .send()
            .await?;

        let status = response.status(); // 保留 status
        let text = response.text().await?; // 消耗 response

        println!("{}", text);
        Ok(status.is_success())
    }

    pub async fn get_courses(&self) -> Result<Vec<ClassBlock>, reqwest::Error> {
        let url = "https://u.unipus.cn/user/student?school_id=10196";
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;

        let data = html_parser::parse_courses_to_json(&html);
        println!("{}", serde_json::to_string_pretty(&data).unwrap());

        Ok(data)
    }

    async fn get_captcha(&self) -> Result<CaptchaResponse, reqwest::Error> {
        let url = "https://sso.unipus.cn/sso/4.0/sso/image_captcha2";
        let response = self.client.post(url).send().await?;
        let response: CaptchaResponse = response.json().await?;
        Ok(response)
    }

    #[allow(dead_code)]
    pub async fn get_course_detail(
        &self,
        tutorial_id: &str,
    ) -> Result<(serde_json::Value, serde_json::Value), reqwest::Error> {
        let url = format!(
            "https://ucontent.unipus.cn/course/api/course/{}/default/",
            tutorial_id
        );
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;

        let course = serde_json::from_str(data["course"].as_str().unwrap()).unwrap();
        Ok((data, course))
    }

    fn save_cookies(&self) {
        let store = self.cookie_store.lock().unwrap();
        let file = std::fs::File::create(&self.cookie_path).unwrap();
        let mut writer = BufWriter::new(file);
        store.save_incl_expired_and_nonpersistent(&mut writer,|c|{
            Ok::<_, cookie_store::Error>(serde_json::to_string(&c).unwrap())
        }).unwrap()
    }

}

impl Drop for Unipus {
    fn drop(&mut self) {
        println!("正在保存 cookie");
        self.save_cookies();
    }
}
