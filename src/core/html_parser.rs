use scraper::{Html, Selector};
use regex::Regex;
use crate::model::class_block::{ClassBlock, Course};

pub fn parse_courses_to_json(html: &str) -> Vec<ClassBlock> {
    let document = Html::parse_document(html);
    let class_selector = Selector::parse(".class-content").unwrap();
    let mut result = Vec::new();

    for class_block in document.select(&class_selector) {
        let class_name = class_block
            .select(&Selector::parse(".class-name").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        let class_date = class_block
            .select(&Selector::parse(".class-date").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("")
            .replace("\n", "")
            .trim()
            .to_string();

        let mut course_list = Vec::new();
        let course_items_selector = Selector::parse(".my_course_item").unwrap();
        for item in class_block.select(&course_items_selector) {
            let course_name = item
                .select(&Selector::parse(".my_course_name").unwrap())
                .next()
                .unwrap()
                .attr("title")
                .unwrap_or("")
                .trim()
                .to_string();
            let status = item
                .select(&Selector::parse(".my_course_status").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string();
            let image_url = item
                .select(&Selector::parse(".my_course_cover").unwrap())
                .next()
                .unwrap()
                .attr("src")
                .unwrap_or("")
                .to_string();
            let course_url = item
                .select(&Selector::parse(".hideurl").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string();
            let tutorial_id = item.value().attr("tutorialid").unwrap_or("").to_string();

            course_list.push(Course {
                course_name,
                status,
                image: image_url,
                course_url,
                tutorial_id,
            });
        }

        // Use regex to extract the date
        let re = Regex::new(r"(\d{1,4}\.\d{1,2}\.\d{1,2})\s.+?\s+(\d{1,4}\.\d{1,2}\.\d{1,2})").unwrap();
        let caps = re.captures(&class_date).unwrap();
        let start_date = caps.get(1).unwrap().as_str().replace(".", "-");
        let end_date = caps.get(2).unwrap().as_str().replace(".", "-");

        result.push(ClassBlock {
            class_name,
            date_range: class_date,
            start_date: start_date,
            end_date: end_date,
            courses: course_list,
        });
    }

    result
}