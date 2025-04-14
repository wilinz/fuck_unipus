use std::io;
use std::io::Write;
use rpassword::read_password;
use crate::utils::input;

pub fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap(); // 确保提示符立即输出

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn input_trim(prompt: &str) -> String {
    input(prompt).trim().to_string()
}

pub fn input_password(prompt: &str) -> String {
    println!("{}", prompt);
    let pass = read_password();
    if pass.is_ok() {
        return pass.unwrap()
    }
    input("")
}

pub fn input_password_trim(prompt: &str) -> String {
    input_password(prompt).trim().to_string()
}