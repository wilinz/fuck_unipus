use fuck_unipus::core::unipus::Unipus;
use fuck_unipus::utils::input::{input, input_password_trim};
use serde_json::Value;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Create Unipus instance
    let username = input("请输入用户名：");
    let mut unipus = Unipus::new(&username);
    let (_, is_authorized) = unipus.check_login_and_setup_session().await.unwrap();
    if is_authorized {
        println!(
            "{} 用户当前已登录",
            &unipus.session_info.as_ref().unwrap().name
        );
        println!("token: {}", &unipus.session_info.as_ref().unwrap().token);
    }
    if !is_authorized {
        let password = input_password_trim("请输入密码：");
        unipus
            .login(&username, &password, None, None)
            .await
            .unwrap();
    }

    let courses = unipus.get_courses().await.unwrap();

    println!(
        "课程信息：{}",
        serde_json::to_string_pretty(&courses).unwrap()
    );

    let tutorial_id = input("请输入课程id：");

    let course_progress = unipus.get_course_progress(&tutorial_id).await.unwrap();

    println!(
        "课程进度：{}",
        serde_json::to_string_pretty(&course_progress).unwrap()
    );

    let course_progress_units = course_progress
        .get("rt")
        .unwrap()
        .get("units")
        .unwrap()
        .as_object()
        .unwrap();

    let mut leafs = HashMap::new();
    for (unit, v) in course_progress_units {
        let unit_progress = unipus
            .get_course_progress_leaf(&tutorial_id, &unit)
            .await
            .unwrap();
        let leafs1 = unit_progress
            .get("rt")
            .unwrap()
            .get("leafs")
            .unwrap()
            .as_object()
            .unwrap();
        for (k, v) in leafs1 {
            leafs.insert(k.clone(), v.clone());
        }
    }

    println!("leafs内容：{}", serde_json::to_string_pretty(&leafs).unwrap());

    let (_, data) = unipus.get_course_detail(&tutorial_id).await.unwrap();

    let units = data.get("units").unwrap().as_array().unwrap();
    traversal_courses(&units, vec![], &unipus, &tutorial_id, &leafs, "");

    let leaf = input("请输入节点 id: ");

    let leaf_content = unipus.get_content(&tutorial_id, &leaf).await.unwrap();

    println!("节点内容：{}", &leaf_content);
}

fn traversal_courses(
    units: &[Value],
    prefix: Vec<usize>,
    unipus: &Unipus,
    tutorial_id: &str,
    leafs_progress: &HashMap<String, Value>,
    tree_prefix: &str, // 新增用于树状符号的前缀
) {
    for (i, unit) in units.iter().enumerate() {
        let is_last = i == units.len() - 1;

        let mut current_prefix = prefix.clone();
        current_prefix.push(i + 1);

        let name = unit
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("<Unnamed>");

        let url = unit.get("url").and_then(|v| v.as_str()).unwrap_or("");

        // 状态检测
        let status_str = if let Some(leaf) = leafs_progress.get(url) {
            let pass = leaf.get("state")
                .and_then(|s| s.get("pass"))
                .and_then(|p| p.as_i64())
                .unwrap_or(0);
            if pass == 1 {
                "✅"
            } else {
                "🕓"
            }
        } else {
            "🚫"
        };

        // 树结构符号
        let branch = if is_last { "└── " } else { "├── " };
        let new_prefix = if is_last {
            format!("{}    ", tree_prefix)
        } else {
            format!("{}│   ", tree_prefix)
        };

        println!("{}{}{} {}", tree_prefix, branch, name, status_str);

        if status_str == "🕓" {
            sleep(Duration::from_secs(3));
        }

        // 递归处理子节点
        if let Some(children) = unit.get("children").and_then(|v| v.as_array()) {
            traversal_courses(
                children,
                current_prefix,
                unipus,
                tutorial_id,
                leafs_progress,
                &new_prefix,
            );
        }
    }
}
