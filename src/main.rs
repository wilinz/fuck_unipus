use fuck_unipus::core::unipus::Unipus;
use fuck_unipus::utils::input::{input, input_password_trim};

#[tokio::main]
async fn main() {
    // Create Unipus instance
    let username = input("请输入用户名：");
    let unipus = Unipus::new(&username);
    let (html, is_authorized) = unipus.get_home_page_and_check_login().await.unwrap();
    let student_name = unipus.extract_name_form_home_page(&html);
    if is_authorized {
        println!("{} 用户当前已登录", &student_name.unwrap_or_default());
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
}

