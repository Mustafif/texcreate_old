use std::borrow::{Borrow, BorrowMut};
use async_std::fs::{File, read_to_string};
use async_std::io::prelude::*;
use async_std::io::stdin;
use texc_config::{to_string_multi, TexCreateError, TexCreateResult, from_str_multi};
use texc_config::{Config, MultiConfig};
use texc_config::TexCreateError::Invalid;

pub async fn init(mode: Option<String>) -> TexCreateResult<()> {
    let mode = match mode {
        Some(m) => m,
        None => "single".to_string(),
    };
    let mut config = Config::default();
    if &mode == "single" {
        // Create single mode config
        println!("Use default settings? (yes/no): ");
        let mut def_settings = String::new();
        stdin().read_line(&mut def_settings).await?;
        return match def_settings.trim() {
            "yes" => {
                let s = config.to_string().unwrap();
                let mut file = File::create("config.toml").await?;
                file.write_all(s.as_bytes()).await?;
                Ok(())
            }
            "no" => {
                let conf = ask_questions().await;
                let s = conf.to_string().unwrap();
                let mut file = File::create("config.toml").await?;
                file.write_all(s.as_bytes()).await?;
                Ok(())
            }
            _ => Err(TexCreateError::Invalid(
                "settings option chosen!".to_string(),
            )),
        };
    } else {
        // Create multi mode config
        let conf1 = Config::default();
        let mut conf2 = Config::default();
        conf2.project_name = "Project2".to_string();
        let multi_config: MultiConfig = vec![conf1, conf2];
        let s = to_string_multi(&multi_config).unwrap();
        let mut file = File::create("config.toml").await?;
        file.write_all(s.as_bytes()).await?;
    }
    Ok(())
}

pub fn update() -> TexCreateResult<()> {
    let cmd = std::process::Command::new("cargo")
        .arg("install")
        .arg("texcreate")
        .output()?;
    if cmd.status.success() {
        println!("{}", String::from_utf8_lossy(&cmd.stdout));
        Ok(())
    } else {
        Err(TexCreateError::Invalid(
            "TexCreate update failed!".to_string(),
        ))
    }
}
fn valid_templates() -> (Vec<&'static str>, Vec<&'static str>) {
    (
        vec![
            "Basic", "Math", "Theatre", "Code", "Novel", "Beamer", "Lachaise",
        ],
        vec![
            "Designed for simple documents",
            "Designed for math focused documents",
            "Designed to write plays/shows",
            "Designed to write code using listings",
            "Designed to write well...novels",
            "Designed to create presentations",
            "Designed for school assignments and academic papers",
        ],
    )
}

pub fn list() {
    let temp = valid_templates();
    println!("=======================");
    println!("  Available Templates  ");
    println!("=======================");
    for t in 0..temp.0.len() {
        println!("==> {}: {}", temp.0[t], temp.1[t]);
    }
    println!("=======================");
}

async fn ask_questions() -> Config {
    println!("Enter Author: ");
    let mut author = String::new();
    stdin().read_line(&mut author).await.unwrap();
    println!("Enter Title: ");
    let mut title = String::new();
    stdin().read_line(&mut title).await.unwrap();
    println!("Enter Date: ");
    let mut date = String::new();
    stdin().read_line(&mut date).await.unwrap();
    println!("Enter Project Name: ");
    let mut project_name = String::new();
    stdin().read_line(&mut project_name).await.unwrap();
    list();
    println!("Enter Template: ");
    let mut template = String::new();
    stdin().read_line(&mut template).await.unwrap();
    println!("Enter Paper Size: ");
    let mut paper_size = String::new();
    stdin().read_line(&mut paper_size).await.unwrap();
    println!("Enter Document Class: ");
    let mut document_class = String::new();
    stdin().read_line(&mut document_class).await.unwrap();
    println!("Enter Font Size: ");
    let mut font_size = String::new();
    stdin().read_line(&mut font_size).await.unwrap();
    println!("Default Project Structure? (y/n): ");
    let mut def_structure = String::new();
    stdin().read_line(&mut def_structure).await.unwrap();
    let def_structure = match def_structure.as_str() {
        "y" => None,
        "n" => Some(true),
        _ => None,
    };
    Config::new(
        author.trim(),
        title.trim(),
        date.trim(),
        project_name.trim(),
        template.trim(),
        paper_size.trim(),
        font_size.trim().parse().unwrap(),
        vec![],
        None,
        def_structure,
        document_class.trim(),
    )
}

fn edit_item(config: &mut Config, field: &Option<String>, field_name: &str, fs:Option<u8>){
    let field = match field.to_owned(){
        Some(f) => f,
        None => "".to_string()
    };
    let fs = match fs {
        Some(f) => f,
        None => 11
    };
    match field_name{
        "author" => config.author = field,
        "title" => config.title = field,
        "date" => config.date = field,
        "rename" => config.project_name = field,
        "template" => config.template = field,
        "paper_size" => config.paper_size = field,
        "font_size" => config.font_size = fs,
        "doc_class" => config.document_class = field,
        "add_package" => {
            let s: String = field;
            config.packages.push(s);
        }
        "rm_package" => {
            let mut v = Vec::new();
            for i in &config.packages{
                if i.as_str() != field.as_str(){
                    v.push(i.to_string());
                }
            }
            config.packages = v;
        }
        _ => println!("Nothing to do")
    }
}

pub async fn edit(
    proj: Option<String>,
    mode: String,
    author: Option<String>,
    title: Option<String>,
    date: Option<String>,
    rename: Option<String>,
    template: Option<String>,
    paper_size: Option<String>,
    font_size: Option<u8>,
    doc_class: Option<String>,
    add_package: Option<String>,
    rm_package: Option<String>,
) -> TexCreateResult<()> {
    match mode.as_str(){
        "multi" => {
            let mut multi_config = from_str_multi(read_to_string("config.toml").await?).unwrap();
            let proj = match proj{
                Some(p) => p,
                None => return Err(Invalid("Project name needs to be provided for multi mode!".to_string()))
            };
            for config in &mut multi_config{
                let config = config.borrow_mut();
                if  &proj == &config.project_name{
                    edit_item(config, &author, "author", None);
                    edit_item(config, &title, "title", None);
                    edit_item(config, &date, "date", None);
                    edit_item(config, &rename, "rename", None);
                    edit_item(config, &template, "template", None);
                    edit_item(config, &paper_size, "paper_size", None);
                    edit_item(config, &doc_class, "doc_class", None);
                    edit_item(config, &None, "font_size", font_size);
                    edit_item(config, &add_package, "add_package", None);
                    edit_item(config, &rm_package, "rm_package", None);
                }
            }
            let mut file = File::create("config.toml").await?;
            file.write_all(to_string_multi(&multi_config).unwrap().as_bytes()).await?;
        },
        "single" => {
            let mut config = Config::from_string(read_to_string("config.toml").await?).unwrap();
            edit_item(&mut config, &author, "author", None);
            edit_item(&mut config, &title, "title", None);
            edit_item(&mut config, &date, "date", None);
            edit_item(&mut config, &rename, "rename", None);
            edit_item(&mut config, &template, "template", None);
            edit_item(&mut config, &paper_size, "paper_size", None);
            edit_item(&mut config, &doc_class, "doc_class", None);
            edit_item(&mut config, &None, "font_size", font_size);
            edit_item(&mut config, &add_package, "add_package", None);
            edit_item(&mut config, &rm_package, "rm_package", None);
            let mut file = File::create("config.toml").await?;
            file.write_all(config.to_string().unwrap().as_bytes()).await?;
        }
        _ => {
            return Err(Invalid("Invalid mode given, only 'single' and 'multi' accepted".to_string()))
        }
    }

    Ok(())
}
