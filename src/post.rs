use chrono::{Utc};
use chrono_tz::US::Central;
use indoc::formatdoc;
use std::fs;

pub struct Post {
    pub title: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub content: String,
}

impl Post {
    fn header(&self) -> String {
        let title = &self.title;
        let tags = &self.tags;
        let categories = &self.categories;
        let date = self.generate_date();
        formatdoc! {
            "---
            title: {title}
            date: {date}
            draft: false
            author: \"Tony\"
            tags: {tags:?}
            categories: {categories:?}
            ---\n\n"
        }
    }

    pub fn render(&self) -> String {
        let header = self.header();
        let body = &self.content;
        formatdoc! {
            "{header}{body}"
        }
    }

   fn generate_date(&self) -> String {
        Utc::now().with_timezone(&Central).to_rfc3339()
   }

    pub fn sanitized_title(&self) -> String {
        let dupe = self.title.clone();
        dupe.replace(" ", "-").to_lowercase()
    }

    pub fn write_to_file(&self, path: &str) {
        println!("writing file to {}", &path);
        fs::write(path, &self.render()).expect("Unable to write file");
    }

}


#[cfg(test)]
mod test {
    use super::*;

    fn test_post() -> Post{
        Post { title: String::from("Test post please ignore"),
               tags: vec![String::from("foo"), String::from("bar")],
               categories: vec![String::from("misc"), String::from("test")],
               content: (String::from("This is a test post wow ok"))
            }
    }

    #[test]
    fn test_sanitized_title(){
        assert_eq!(test_post().sanitized_title(), "test-post-please-ignore");
    }

    #[test]
    fn test_date_generation(){
        let re = Regex::new(r"^((?:(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?))(Z|[\+-]\d{2}:\d{2})?)$").unwrap();
        let date = test_post().generate_date();
        assert!(re.is_match(&date));
    }

}