use serde_derive::Deserialize;
use std::io::prelude::*;
use toml::from_str;
use super::{template::Template, consts::*};
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub Project: Project,
    pub Document: Document,
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
            "Code" => Template::Code,
            "Novel" => Template::Novel,
            "Beamer" => Template::Beamer,
            "Lachaise" => Template::Lachaise,
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
        // For Beamer class only, panics if doc class is not Beamer
        if self.Project.template == "Beamer" && self.Document.document_class != "beamer" {
            eprintln!("Beamer Template must have a document class as beamer!!!");
            return;
        }
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