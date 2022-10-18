use chrono::{Utc};
use chrono_tz::US::Central;
use std::fs;
use textwrap::dedent;

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
        let date = Utc::now().with_timezone(&Central).to_rfc3339();
dedent(&format!("---
title: {title}
date: {date}
draft: false
author: \"Tony\"
tags: {tags:?}
categories: {categories:?}
---\n\n"))
    }

    pub fn render(&self) -> String {
        let header = self.header();
        let body = &self.content;
        {
dedent(&format!("
{header}
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