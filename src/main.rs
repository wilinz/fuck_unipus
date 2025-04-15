use async_recursion::async_recursion;
use fuck_unipus::core::unipus::Unipus;
use fuck_unipus::utils::input::{input, input_password_trim};
use futures::FutureExt;
use futures::future::BoxFuture;
use rand::{Rng, random, rng};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::thread::sleep;
use std::time::Duration;
use tokio::fs;
use fuck_unipus::error::unipus::UnipusError;

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

    println!("📚 课程信息如下：");
    for class in &courses {
        println!("\n============================================================");
        println!("🔹 班级名称: {}", class.class_name);
        println!("📆 时间范围: {}", class.date_range);
        for course in &class.courses {
            println!("  ─────────────────────────────────────────────");
            println!("  📖 课程名称: {}", course.course_name);
            println!("  🔗 状态: {}", course.status);
            println!("  🆔 tutorial_id: {}", course.tutorial_id);
            println!("  🌐 链接: {}", course.course_url);
        }
        println!("============================================================\n");
    }

    // 接收输入的 tutorial_id
    let tutorial_id = input("请输入课程的 tutorial_id：");

    let course_progress = unipus.get_course_progress(&tutorial_id).await.unwrap();

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

    let (_, data) = unipus.get_course_detail(&tutorial_id).await.unwrap();

    let units = data.get("units").unwrap().as_array().unwrap();
    let _ = traversal_courses_to_fs(&units, vec![], &unipus, &tutorial_id, &leafs, "", Path::new("courses")).await;

    let leaf = input("请输入节点 id: ");

    let leaf_content = unipus
        .get_course_leaf_content(&tutorial_id, &leaf)
        .await
        .unwrap();

    println!("节点内容：{}", &leaf_content);
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim()
        .replace(' ', "_")
}

pub async fn traversal_courses_to_fs(
    units: &[Value],
    prefix: Vec<usize>,
    unipus: &Unipus,
    tutorial_id: &str,
    leafs_progress: &HashMap<String, Value>,
    tree_prefix: &str,
    root_dir: &Path,
) -> Result<(), UnipusError> {
    fs::create_dir_all(root_dir).await?;
    traversal_courses_inner(
        units,
        prefix,
        unipus,
        tutorial_id,
        leafs_progress,
        tree_prefix,
        root_dir.to_path_buf(),
    ).await
}

#[async_recursion]
async fn traversal_courses_inner(
    units: &[Value],
    prefix: Vec<usize>,
    unipus: &Unipus,
    tutorial_id: &str,
    leafs_progress: &HashMap<String, Value>,
    tree_prefix: &str,
    current_path: PathBuf,
) -> Result<(), UnipusError> {
    for (i, unit) in units.iter().enumerate() {
        let is_last = i + 1 == units.len();
        let mut current_prefix = prefix.clone();
        current_prefix.push(i + 1);

        let name = unit.get("name").and_then(Value::as_str).unwrap_or("<Unnamed>");
        let url = unit.get("url").and_then(Value::as_str).unwrap_or("");

        let status_str = match leafs_progress.get(url) {
            Some(leaf) => {
                let pass = leaf.get("state").and_then(|s| s.get("pass")).and_then(|p| p.as_i64()).unwrap_or(0);
                let required = leaf.get("strategies").and_then(|s| s.get("required")).and_then(|p| p.as_bool()).unwrap_or(false);
                if pass == 1 {
                    "✅"
                } else if required {
                    "Fucking... 🕓"
                } else {
                    "🚫"
                }
            }
            None => "",
        };

        let branch = if is_last { "└── " } else { "├── " };
        println!("{}{}{} {}", tree_prefix, branch, name, status_str);

        let dir_name = format!("{}.{}", i+1, sanitize_filename(name));
        let this_path = current_path.join(&dir_name);
        fs::create_dir_all(&this_path).await?;

        if status_str == "Fucking... 🕓" {
            let Ok(content) = unipus.get_course_leaf_content(tutorial_id, url).await else {
                eprintln!("获取内容失败：{}", url);
                continue;
            };

            let Ok(questions) = unipus.get_course_leaf_questions(tutorial_id, url).await else {
                eprintln!("获取问题失败：{}", url);
                continue;
            };

            let Ok(content_pretty) = serde_json::to_string_pretty(&content) else {
                eprintln!("格式化 JSON 失败：{}", url);
                continue;
            };

            let Ok(questions_pretty) = serde_json::to_string_pretty(&questions) else {
                eprintln!("格式化 JSON 失败：{}", url);
                continue;
            };

            let file_path = this_path.join("content.json5");
            // let spaces = std::iter::repeat(' ').take(tree_prefix.len()).collect::<String>();
            println!("{}{}Fucking content ... {}", tree_prefix, branch, file_path.display());
            if let Err(e) = fs::write(&file_path, content_pretty).await {
                eprintln!("写入失败 {}: {}", file_path.display(), e);
            }

            let file_path = this_path.join("questions.json5");
            // let spaces = std::iter::repeat(' ').take(tree_prefix.len()).collect::<String>();
            println!("{}{}Fucking questions ... {}", tree_prefix, branch, file_path.display());
            if let Err(e) = fs::write(&file_path, questions_pretty).await {
                eprintln!("写入失败 {}: {}", file_path.display(), e);
            }

            // let sleep_time = rng().random_range(3..=10);
            // tokio::time::sleep(Duration::from_secs(sleep_time)).await;
        }

        if let Some(children) = unit.get("children").and_then(Value::as_array) {
            let new_prefix = if is_last {
                format!("{}    ", tree_prefix)
            } else {
                format!("{}│   ", tree_prefix)
            };
            traversal_courses_inner(
                children,
                current_prefix.clone(),
                unipus,
                tutorial_id,
                leafs_progress,
                &new_prefix,
                this_path,
            ).await?;
        }
    }

    Ok(())
}
