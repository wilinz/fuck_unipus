use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct SsoLoginResponse {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "msg")]
    pub msg: String,

    #[serde(rename = "error")]
    pub error: Option<String>,

    #[serde(rename = "rs")]
    pub rs: Option<SsoResult>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct SsoResult {
    #[serde(rename = "grantingTicket")]
    pub granting_ticket: String,

    #[serde(rename = "serviceTicket")]
    pub service_ticket: String,

    #[serde(rename = "tgtExpiredTime")]
    pub tgt_expired_time: i64,

    #[serde(rename = "role")]
    pub role: Option<String>,

    #[serde(rename = "openid")]
    pub openid: String,

    #[serde(rename = "nickname")]
    pub nickname: String,

    #[serde(rename = "fullname")]
    pub fullname: Option<String>,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "mobile")]
    pub mobile: String,

    #[serde(rename = "email")]
    pub email: Option<String>,

    #[serde(rename = "perms")]
    pub perms: String,

    #[serde(rename = "isSsoLogin")]
    pub is_sso_login: String,

    #[serde(rename = "isCompleted")]
    pub is_completed: Option<String>,

    #[serde(rename = "openidHash")]
    pub openid_hash: Option<String>,

    #[serde(rename = "jwt")]
    pub jwt: String,

    #[serde(rename = "rt")]
    pub rt: String,

    #[serde(rename = "createTime")]
    pub create_time: Option<String>,

    #[serde(rename = "status")]
    pub status: i32,

    #[serde(rename = "source")]
    pub source: Option<String>,

    #[serde(rename = "links")]
    pub links: Vec<SsoLink>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct SsoLink {
    #[serde(rename = "rel")]
    pub rel: String,

    #[serde(rename = "href")]
    pub href: String,
}
