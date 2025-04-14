use rand::seq::{IndexedRandom};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone ,Serialize, Deserialize, Debug)]
pub struct PlatformInfo {
    pub user_agent_platform: &'static str,
    pub sec_ch_ua_platform: &'static str,
    pub is_mobile: bool,
}

pub fn get_random_platform_info() -> PlatformInfo {
    let platforms = [
        PlatformInfo {
            user_agent_platform: "Windows NT 10.0; Win64; x64",
            sec_ch_ua_platform: "Windows",
            is_mobile: false,
        },
        PlatformInfo {
            user_agent_platform: "Macintosh; Intel Mac OS X 10_15_7",
            sec_ch_ua_platform: "macOS",
            is_mobile: false,
        },
        PlatformInfo {
            user_agent_platform: "X11; Linux x86_64",
            sec_ch_ua_platform: "Linux",
            is_mobile: false,
        },
        PlatformInfo {
            user_agent_platform: "Android 10; Mobile",
            sec_ch_ua_platform: "Android",
            is_mobile: true,
        },
        PlatformInfo {
            user_agent_platform: "iPhone; CPU iPhone OS 14_0 like Mac OS X",
            sec_ch_ua_platform: "iOS",
            is_mobile: true,
        },
    ];

    let mut rng = rand::rng();
    platforms.choose(&mut rng).unwrap().to_owned()
}

pub fn get_random_user_agent(platform_info: &PlatformInfo) -> String {
    let mut rng = rand::rng();

    let chrome_version = format!("Chrome/{}.0.0.0", 80 + rng.random_range(0..30));
    let firefox_version = format!("Firefox/{}.0", 70 + rng.random_range(0..20));
    let safari_version = "Safari/537.36".to_string();
    let edge_version = format!("Edge/{}.0.0.0", 90 + rng.random_range(0..10));

    let browsers = [
        chrome_version,
        firefox_version,
        safari_version,
        edge_version,
    ];

    let browser = browsers.choose(&mut rng).unwrap();

    format!(
        "Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) {}",
        platform_info.user_agent_platform, browser
    )
}