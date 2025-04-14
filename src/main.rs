use fuck_unipus::core::unipus::Unipus;
use fuck_unipus::utils::input::{input, input_password_trim};

#[tokio::main]
async fn main() {
    // Create Unipus instance
    let username = input("请输入用户名：");
    let mut unipus = Unipus::new(&username);
    let (_, is_authorized) = unipus.get_home_page_and_check_login().await.unwrap();
    if is_authorized {
        println!("{} 用户当前已登录", &unipus.session_info.as_ref().unwrap().name);
        println!("token: {}", &unipus.session_info.as_ref().unwrap().token);
    }
    if !is_authorized {
        let password = input_password_trim("请输入密码：");
        let Ok(_) = unipus.login(&username, &password, None, None).await else {
            eprintln!("登录失败");
            return;
        };
    }

    let courses = unipus.get_courses().await.unwrap();

    println!("课程信息：{}", serde_json::to_string_pretty(&courses).unwrap());

    let tutorial_id = input("请输入课程id：");

    let (_, data) = unipus.get_course_detail(&tutorial_id).await.unwrap();

    println!("课程详细信息：{}", serde_json::to_string_pretty(&data).unwrap());

    let leaf = input("请输入节点 id: ");

    let leaf_content = unipus.get_content(&tutorial_id, &leaf).await.unwrap();

    println!("节点内容：{}", &leaf_content);

}

