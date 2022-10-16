use chrono::{Utc};

pub struct Post {
    pub title: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub content: String,
}

impl Post {
    pub fn header(&self) -> String {
        let title = &self.title;
        let tags = &self.tags;
        let categories = &self.categories;
        let date = Utc::now().to_string();
        {
            format!("
            ---
            title: {title}\n
            date: {date}\n
            draft: false\n
            author: \"Tony\"
            tags: {tags:?}
            categories: {categories:?}
            ---")
        }
    }
}