use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Course {
    pub course_name: String,
    pub status: String,
    pub image: String,
    pub course_url: String,
    pub tutorial_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct ClassBlock {
    pub class_name: String,
    pub date_range: String,
    pub courses: Vec<Course>,
}