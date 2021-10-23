use serde_derive::Deserialize;
use toml::from_str;
use std::io::prelude::*;
pub const AUTHOR: &str = "\\author{}";
pub const TITLE: &str = "\\title{}";
pub const DATE: &str = "\\date{}";

#[derive(Deserialize)]
pub struct Config{
    pub Project:Project,
}
#[derive(Deserialize)]
pub enum Template{
    Basic, 
    Math, 
    Book    
}
#[derive(Deserialize)]
pub struct Project{
    pub author: String,
    pub title: String,
    pub date: String,
    pub template: String,
    pub project_name: String,
}

impl Config{
    pub fn from_template(&self)-> Template{
        match self.Project.template.as_str(){
            "Basic" => Template::Basic,
            "Math" => Template::Math,
            "Book" => Template::Book,
            _ => Template::Basic
        }
    }
    
    pub fn config(path: &str) -> Self{
        let config = std::fs::read_to_string(path).unwrap();
        let config: Self = from_str(&config).unwrap();
        config
    }
    pub fn adjust(&self, path: &str){
        let title = format!("\\title{{{}}}", self.Project.title);
        let author = format!("\\author{{{}}}", self.Project.author);
        let date = format!("\\date{{{}}}", self.Project.date);

        let mut file = std::fs::File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut content = content.replace(AUTHOR, &author);
        content = content.replace(TITLE, &title);
        content = content.replace(DATE, &date);
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}



