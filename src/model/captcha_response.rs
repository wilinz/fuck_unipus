use serde::{Deserialize, Serialize};

/// 顶层响应结构
#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct CaptchaResponse {
    pub code: String,
    pub msg: String,
    pub rs: Rs,
}

/// 响应数据体
#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Rs {
    /// 原始字段名为 "type"
    #[serde(rename = "type")]
    pub type_field: String,

    pub image: String,

    /// 原始字段名为 "encodeCaptha"
    #[serde(rename = "encodeCaptha")]
    pub encode_captcha: String,

    /// 原始字段名为 "codeType"
    #[serde(rename = "codeType")]
    pub code_type: u32,

    pub links: Vec<Link>,
}

/// 超链接信息
#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Link {
    pub rel: String,
    pub href: String,
}
