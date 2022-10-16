use chrono::{Utc};
use std::fs;
use textwrap::dedent;

pub struct Post {
    pub title: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub content: String,
}

impl Post {
    pub fn render(&self) -> String {
        let title = &self.title;
        let tags = &self.tags;
        let categories = &self.categories;
        let date = Utc::now().to_string();
        let body = &self.content;
        {
            dedent(&format!("
---
title: {title}\n
date: {date}\n
draft: false\n
author: \"Tony\"
tags: {tags:?}
categories: {categories:?}
---\n\n
{body}"))
        }
    }

    pub fn sanitized_title(&self) -> String {
        let dupe = self.title.clone();
        dupe.replace(" ", "-")
    }

    pub fn write_to_file(&self, path: &str) {
        println!("writing file to {}", &path);
        fs::write(path, &self.render()).expect("Unable to write file");
    }

}