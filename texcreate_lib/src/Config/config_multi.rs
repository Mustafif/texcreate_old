use super::{consts::*, template::Template};
use crate::config::Config;
use crate::error::Errors;
use crate::{invalid_class, load};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::create_dir;
use std::io::prelude::*;
use std::path::Path;
use toml::from_str;
use toml::to_string_pretty;

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigMulti {
    pub conf: Vec<Config>,
}

pub fn from_template(temp: &str) -> Template {
    match temp {
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

impl Default for ConfigMulti {
    fn default() -> Self {
        let conf_one = Config::default();
        let mut conf_two = Config::default();
        conf_two.Project.project_name = "Project2".to_owned();
        return ConfigMulti {
            conf: vec![conf_one, conf_two],
        };
    }
}

impl ConfigMulti {
    pub fn init() -> Result<(), std::io::Error> {
        let mut file = fs::File::create("config.toml")?;
        let s = to_string_pretty(&ConfigMulti::default()).unwrap();
        file.write_all(s.as_bytes())?;
        Ok(())
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

    pub fn adjust(&self, file_path: &Option<String>) -> Result<(), Errors> {
        //Create portion
        //Sanity check file path
        let mut path = String::new();
        let mut main_path: Vec<String> = Vec::new();
        let mut str_path: Vec<String> = Vec::new();

        let file_path = match file_path {
            Some(path) => path,
            None => ".",
        };
        let mut title: Vec<String> = Vec::new();
        let mut author: Vec<String> = Vec::new();
        let mut date: Vec<String> = Vec::new();

        let mut papersize: Vec<String> = Vec::new();
        let mut fontsize: Vec<String> = Vec::new();
        let mut doc_class: Vec<String> = Vec::new();
        let len = &self.conf.len();
        for i in &self.conf {
            println!("Loading {} template...", &i.Project.template);
            let (main, structure) = load(&i.Project.template);
            path = format!("{}/{}", file_path, &i.Project.project_name);
            let def_path = format!("{}/{}", file_path, &i.Project.project_name);

            if Path::new(&path).exists() {
                let f = format!("Note {} exists, creating {} instead\n", &path, &def_path);
                println!("{}", f);
                create_dir(&def_path)?;
                path = def_path;
            } else {
                println!("Creating {}", &path);
                create_dir(&path)?;
            }

            //Creating the main file
            println!("Creating {}/{}.tex", &path, &i.Project.project_name);
            let mut main_file =
                std::fs::File::create(format!("{}/{}.tex", &path, &i.Project.project_name))?;
            main_file.write_all(main.as_bytes())?;
            //Creating structure.tex
            println!("Creating {}/structure.tex\n", &path);
            let mut structure_file = std::fs::File::create(format!("{}/structure.tex", &path))?;
            structure_file.write_all(structure.as_bytes())?;
            main_path.push(format!("{}/{}.tex", &path, &i.Project.project_name));
            str_path.push(format!("{}/structure.tex", &path));
            //Check for errors
            // Checking for invalid templates
            let l = Template::list();

            if !l.contains(&i.Project.template) {
                let inv_temp = i.Project.template.clone();
                return Err(Errors::InvalidTemplateError(inv_temp));
            }
            //Check for empty project
            if &i.Project.author == "" {
                return Err(Errors::EmptyError("Author".to_string()));
            } else if &i.Project.title == "" {
                return Err(Errors::EmptyError("Title".to_string()));
            } else if &i.Project.date == "" {
                return Err(Errors::EmptyError("Date".to_string()));
            } else if &i.Project.project_name == "" {
                return Err(Errors::EmptyError("Project Name".to_string()));
            } else if &i.Project.template == "" {
                return Err(Errors::EmptyError("Template".to_string()));
            }
            //Check for Invalid class error
            if invalid_class(&i.Document.document_class) {
                return Err(Errors::InvalidDocumentClassError(
                    i.Document.document_class.clone(),
                ));
            }
            papersize.push(i.Document.paper_size.to_string());
            fontsize.push(i.Document.font_size.to_string());
            doc_class.push(i.Document.document_class.to_string());
            //After errors push formatted title, author, date
            title.push(i.Project.title.clone());
            author.push(i.Project.author.clone());
            date.push(i.Project.date.clone());
        }
        for k in 0..main_path.len() {
            let mut file = std::fs::File::open(&main_path[k])?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            //Project Adjustments
            println!("Adjusting author for {}", &main_path[k]);
            content = content.replace("{author}", &author[k]);
            println!("Adjusting title for {}", &main_path[k]);
            content = content.replace("{title}", &title[k]);
            println!("Adjusting date for {}", &main_path[k]);
            content = content.replace("{date}", &date[k]);
            //Document Adjustments
            println!("Adjusting papersize for {}", &main_path[k]);
            content = content.replace("{paper_size}", &papersize[k]);
            println!("Adjusting fontsize for {}", &main_path[k]);
            content = content.replace("{font_size}", &fontsize[k]);
            println!("Adjusting document's class for {}\n", &main_path[k]);
            content = content.replace("{doc_class}", &doc_class[k]);

            let mut file = std::fs::File::create(&main_path[k]).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
        // Package adjustments
        let mut packages: Vec<Vec<String>> = Vec::new();
        for i in &self.conf {
            packages.push(i.Document.packages.to_owned());
        }
        if packages.len() < 1 {
            println!("No packages to add...");
            return Ok(());
        }
        for k in 0..str_path.len() {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&str_path[k])
                .expect("Couldn't open file");
                let i: Vec<String> = packages[k].iter().map(|x| format!("\\usepackage{{{}}}", x)).collect();
                let i = i.join("\n");
                println!("Adding packages to {}", &str_path[k] );
                write!(file, "{}", i)?;
        }
        Ok(())
    }
}
