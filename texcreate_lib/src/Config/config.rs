use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use toml::{from_str, to_string_pretty};
use super::{template::Template, consts::*};
use std::fs;
use crate::error::Errors;
use crate::invalid_class;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub Project: Project,
    pub Document: Document,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Project {
    pub author: String,
    pub title: String,
    pub date: String,
    pub template: String,
    pub project_name: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Document {
    pub paper_size: String,
    pub font_size: u8,
    pub document_class: String,
    pub packages: Vec<String>,
}

impl Project{
    pub fn new(author: String, title: String, date: String, template: String, project_name: String)
        -> Project {
        Project {
            author,
            title,
            date,
            template,
            project_name,
        }
    }
}

impl Document{
    pub fn new(paper_size: String, font_size: u8, document_class: String, packages: Vec<String>)
        -> Document {
        Document {
            paper_size,
            font_size,
            document_class,
            packages,
        }
    }
}

impl Default for Config{
    fn default() -> Config{
        let proj = Project::new("author".to_owned(), "title".to_owned(), "\\\\today".to_owned(),
                                "Basic".to_owned(), "Project".to_owned());
        let doc = Document::new("letterpaper".to_owned(), 11, "article".to_owned(), vec![]);
        Config::new(proj, doc)
    }
}


impl Config {
    pub fn new(Project: Project, Document: Document) -> Config {
        Config {
            Project,
            Document,
        }
    }
    pub fn init(conf: Option<Self>) -> Result<(), std::io::Error>{
        let mut file = fs::File::create("config.toml")?;
        let conf = match conf{
            Some(c) => c,
            None => Config::default()
        };
        file.write_all(to_string_pretty(&conf).unwrap().as_bytes())?;
        Ok(())
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
    pub fn adjust(&self, path: &str) -> Result<(), Errors> {
        //Check all the configs for errors
        //Checking for invalid template
        let l = Template::list();
        if !l.contains(&self.Project.template){
            let inv_temp = self.Project.template.clone();
            return Err(Errors::InvalidTemplateError(inv_temp))
        }
        //Checking for empty project
        let proj = &self.Project;
        if proj.author == "".to_string(){
            return Err(Errors::EmptyError("Author".to_string()))
        } else if proj.title == "".to_string(){
            return Err(Errors::EmptyError("Title".to_string()))
        } else if proj.date == "".to_string(){
            return Err(Errors::EmptyError("Date".to_string()))
        } else if proj.project_name == "".to_string(){
            return Err(Errors::EmptyError("Project Name".to_string()))
        } else if proj.template == "".to_string(){
            return Err(Errors::EmptyError("Template".to_string()))
        }
        // Checking for invalid doc class
        if invalid_class(&self.Document.document_class){
            return Err(
                Errors::InvalidDocumentClassError(self.Document.document_class.clone())
            )
        }

        let mut file = std::fs::File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // Project adjustments
        println!("Adjusting author...");
        let mut content = content.replace("{author}", &self.Project.author);
        println!("Adjusting title...");
        content = content.replace("{title}", &self.Project.title);
        println!("Adjusting date...");
        content = content.replace("{date}", &self.Project.date);
        // Document adjustments
        println!("Adjusting document's papersize...");
        content = content.replace("{paper_size}", &self.Document.paper_size);
        println!("Adjusting document's fontsize...");
        content = content.replace("{font_size}", &self.Document.font_size.to_string());
        // For Beamer class only, panics if doc class is not Beamer
        if self.Project.template == "Beamer" && self.Document.document_class != "beamer" {
            return Err(Errors::BeamerError)
        }
        println!("Adjusting document's class");
        content = content.replace("{doc_class}", &self.Document.document_class);
        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
    pub fn add_packages(&self, path: &str) {
        if self.Document.packages.len() < 1 {
            println!("No packages to add...");
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
        println!("Adding packages to structure.tex");
        write!(file, "{}", p).expect("Couldn't write to file");
    }
}