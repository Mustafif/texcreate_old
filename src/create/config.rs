use super::routes::*;
use serde_derive::Deserialize;
use serde_json::Error;
use std::fs;
use std::io::prelude::*;
use toml::from_str;
pub const AUTHOR: &str = "\\author{}";
pub const TITLE: &str = "\\title{}";
pub const DATE: &str = "\\date{}";

#[derive(Deserialize)]
pub struct Config {
    pub Project: Project,
    pub Document: Document,
}
#[derive(Deserialize)]
pub enum Template {
    Basic,
    Math,
    Theatre,
    Book,
}
#[derive(Deserialize)]
pub struct Project {
    pub author: String,
    pub title: String,
    pub date: String,
    pub template: String,
    pub project_name: String,
}
#[derive(Deserialize)]
pub struct Document {
    pub paper_size: String,
    pub font_size: u8,
    pub document_class: String,
    pub packages: Vec<String>,
}
#[derive(Deserialize)]
pub struct List {
    pub template: String,
    pub about: String,
}

impl List {
    pub fn read(path: &str) -> Result<Vec<Self>, Error> {
        let mut file = fs::File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        let list: Vec<List> = serde_json::from_str(&contents)?;
        Ok(list)
    }
    pub fn list(path: &str) {
        let json = Self::read(path).expect("Unable to read list.json");
        println!("//////////////////////////////////////");
        println!("// List of available templates:");
        println!("//////////////////////////////////////");
        for item in json {
            println!("// {} => {}", item.template, item.about);
        }
    }
}

impl Config {
    pub fn init() {
        let mut file = fs::File::create("config.toml").expect("Unable to create config file");
        file.write_all(CONFIG_TOML.as_bytes())
            .expect("Unable to write to config file");
    }
    pub fn from_template(&self) -> Template {
        match self.Project.template.as_str() {
            "Basic" => Template::Basic,
            "Math" => Template::Math,
            "Theatre" => Template::Theatre,
            "Book" => Template::Book,
            _ => Template::Basic,
        }
    }
    pub fn config(path: &Option<String>) -> Self {
        match path {
            Some(path) => {
                let config = std::fs::read_to_string(path).expect("Unable to read config file");
                from_str(&config).expect("Unable to parse config file")
            }
            None => {
                let config =
                    std::fs::read_to_string("config.toml").expect("Unable to read config file");
                from_str(&config).expect("Unable to parse config file")
            }
        }
    }
    pub fn adjust(&self, path: &str) {
        let title = format!("\\title{{{}}}", self.Project.title);
        let author = format!("\\author{{{}}}", self.Project.author);
        let date = format!("\\date{{{}}}", self.Project.date);

        let mut file = std::fs::File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        // Project adjustments
        let mut content = content.replace(AUTHOR, &author);
        content = content.replace(TITLE, &title);
        content = content.replace(DATE, &date);
        // Document adjustments
        content = content.replace("letterpaper", &self.Document.paper_size);
        content = content.replace("11pt", &format!("{}pt", &self.Document.font_size));
        content = content.replace("article", &self.Document.document_class);
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
    pub fn add_packages(&self, path: &str) {
        if self.Document.packages.len() < 1 {
            return;
        }

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .expect("Couldn't append to file");

        let p: Vec<String> = self
            .Document
            .packages
            .iter()
            .map(|x| format!("\\usepackage{{{}}}", x))
            .collect();
        let p = p.join("\n");
        write!(file, "{}", p).expect("Couldn't write to file");
    }
}

const CONFIG_TOML: &str = r#"[Project]
author = "Author"
title = "Title"
date = "\\today"
project_name = "Project Name"
template = "Math" #Make sure to have first letter upercased

[Document]
paper_size = "letterpaper"
font_size = 11 #font size number
document_class = "article"
packages = ["PhantomData", ""]
"#;
