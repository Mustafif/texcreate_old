use serde_derive::Deserialize;
use std::io::prelude::*;
use toml::from_str;
use super::{template::Template, consts::*};
use std::fs;
use std::fs::create_dir;
use std::path::Path;
use crate::error::Errors;
use crate::config::{Project, Document};
use crate::load;


#[derive(Deserialize)]
pub struct Config_Multi{
    pub Project: Vec<Project>,
    pub Document: Vec<Document>,
}

pub fn from_template(temp: &str) -> Template {
    match temp{
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

impl Config_Multi{
    pub fn init(){
        let mut file = fs::File::create("config.toml").expect("Unable to create config.toml");
        file.write_all(CONFIG_TOML_MULTI.as_bytes())
            .expect("Unable to write config.toml");
    }
    pub fn config(path: &Option<String>)->Self{
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

    pub fn adjust(&self, file_path: &Option<String>)->Result<(), Errors>{
        //Create portion
        //Sanity check file path
        let mut path = String::new();
        let mut main_path: Vec<String> = Vec::new();
        let mut str_path: Vec<String> = Vec::new();

        let file_path = match file_path{
            Some(path)=> path,
            None => "."
        };
        for i in &self.Project{
            println!("Loading {} template...", i.template);
            let (main, structure) = load(&i.template);
            path = format!("{}/{}", file_path, &i.project_name);
            let def_path = format!("{}/{}", file_path, &i.project_name);

            if Path::new(&path).exists(){
                let f = format!("Note {} exists, creating {} instead\n" ,&path, &def_path);
                println!("{}", f);
                create_dir(&def_path).expect("Error in creating directory");
                path = def_path;
            } else {
                println!("Creating {}", &path);
                create_dir(&path).expect("Error in creating directory");
            }

            //Creating the main file
            println!("Creating {}/{}.tex", &path, &i.project_name);
            let mut main_file =
                std::fs::File::create(format!("{}/{}.tex", &path, &i.project_name)).unwrap();
            main_file.write_all(main.as_bytes()).unwrap();
            //Creating structure.tex
            println!("Creating {}/structure.tex", &path);
            let mut structure_file =
                std::fs::File::create(format!("{}/structure.tex", &path)).unwrap();
            structure_file.write_all(structure.as_bytes()).unwrap();
            main_path.push(format!("{}/{}.tex", &path, &i.project_name));
            str_path.push(format!("{}/structure.tex", &path));
        }
        //Check for errors
        // Checking for invalid templates
        let l = Template::list();
        let mut title: Vec<String> = Vec::new();
        let mut author: Vec<String> = Vec::new();
        let mut date: Vec<String> = Vec::new();

        for i in &self.Project{
            if !l.contains(&i.template){
                let inv_temp = i.template.clone();
                return Err(Errors::InvalidTemplateError(inv_temp));
            }
            //Check for empty project
            if i.author == "".to_string(){
                return Err(Errors::EmptyError);
            } else if i.title == "".to_string(){
                return Err(Errors::EmptyError);
            } else if i.date == "".to_string(){
                return Err(Errors::EmptyError);
            } else if i.project_name == "".to_string(){
                return Err(Errors::EmptyError);
            } else if i.template == "".to_string(){
                return Err(Errors::EmptyError);
            }
            //After errors push formatted title, author, date
            title.push(format!("\\title{{{}}}", &i.title));
            author.push(format!("\\author{{{}}}", &i.author));
            date.push(format!("\\date{{{}}}", &i.date))
        }

        let mut papersize: Vec<String> = Vec::new();
        let mut fontsize: Vec<String> = Vec::new();
        let mut doc_class: Vec<String> = Vec::new();
        //Document variables
        for i in &self.Document{
            papersize.push(i.paper_size.to_string());
            fontsize.push(i.font_size.to_string());
            doc_class.push(i.document_class.to_string());
        }

        for k in main_path{
            let mut file = std::fs::File::open(&k).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            //Project Adjustments
            for i in 0..title.len(){
                println!("Adjusting author...");
                content = content.replace(AUTHOR, &author[i]);
                println!("Adjusting title...");
                content = content.replace(TITLE, &title[i]);
                println!("Adjusting date...");
                content = content.replace(DATE, &date[i]);
            }

            //Document Adjustments
            for i in 0..doc_class.len(){
                println!("Adjusting papersize...");
                content = content.replace("letterpaper", &papersize[i]);
                println!("Adjusting fontsize...");
                content = content.replace("11pt", &format!("{}pt", &fontsize[i]));
                println!("Adjusting document's class...");
                content = content.replace("article", &doc_class[i]);
            }
            let mut file = std::fs::File::create(&k).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
        // Package adjustments
        let mut packages: Vec<Vec<String>> = Vec::new();
        for i in &self.Document{
            packages.push(i.packages.to_owned());
        }
        if packages.len() < 1 {
            println!("No packages to add...");
            return Ok(());
        }
        for k in str_path{
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&k)
                .expect("Couldn't open file");
            for i in &packages{
                let i: Vec<String> = i.iter()
                    .map(|x| format!("\\usepackage{{{}}}", x))
                    .collect();
                let i = i.join("\n");
                println!("Adding packages to structure.tex");
                write!(file, "{}", i).expect("Couldn't write to file");
            }
        }
        Ok(())
    }
}