//! TexCreate Utilities Library <br>
//! This library is intended to contain functions for subcommands <br>
//! Developer: Mustafif Khan <br>
//! Project: TexCreate  <br>
//! License: MIT & GPLv2 <br>


use async_std::fs::{File, read_to_string};
use async_std::io::prelude::*;
use async_std::io::stdin;
use texc_config::{TexCreateError, TexCreateResult};
use texc_config::Config;

/// Initializes a TexCreate Project
pub async fn init() -> TexCreateResult<()> {
    let config = Config::default();
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
}

/// Updates TexCreate
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
            "Basic", "Math", "Theatre", "Code", "Novel", "Beamer", "Lachaise", "Lachaise-Mod", "Dictionary", "News"
        ],
        vec![
            "Designed for simple documents",
            "Designed for math focused documents",
            "Designed to write plays/shows",
            "Designed to write code using listings",
            "Designed to write well...novels",
            "Designed to create presentations",
            "Designed for school assignments and academic papers",
            "Modified version of Lachaise Template, changes in colours/design",
            "Designed to write your own dictionary",
            "Designed to write columned newspapers"
        ],
    )
}
/// Lists all templates
pub fn list() {
    let temp = valid_templates();
    println!("=======================");
    println!("  Available Templates  ");
    println!("=======================");
    for t in 0..temp.0.len() {
        println!("==> {}: {}", temp.0[t], temp.1[t.clone()]);
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
        &Some(template),
        &None,
        paper_size.trim(),
        font_size.trim().parse().unwrap(),
        vec![],
        def_structure,
        document_class.trim(),
    )
}

fn edit_item(config: &mut Config, field: &Option<String>, field_name: &str, fs: Option<u8>) {
    let field = match field.to_owned() {
        Some(f) => f,
        None => return println!("No changes to {}", field_name),
    };
    let fs = match fs {
        Some(f) => f,
        None => 11,
    };
    match field_name {
        "author" => {
            println!(
                "Changed {} from {} to {}",
                field_name, &config.author, &field
            );
            config.author = field;
        }
        "title" => {
            println!(
                "Changed {} from {} to {}",
                field_name, &config.title, &field
            );
            config.title = field;
        }
        "date" => {
            println!("Changed {} from {} to {}", field_name, &config.date, &field);
            config.date = field
        }
        "rename" => {
            println!("Renamed project to {}", &field);
            config.project_name = field
        }
        "template" => {
            println!(
                "Changed {} from {} to {}",
                field_name, &config.template.as_ref().unwrap(), &field
            );
            config.template = Some(field)
        }
        "paper_size" => {
            println!(
                "Changed {} from {} to {}",
                field_name, &config.paper_size, &field
            );
            config.paper_size = field
        }
        "font_size" => {
            println!(
                "Changed {} from {} to {}",
                field_name,
                &config.font_size.to_string(),
                &field
            );
            config.font_size = fs
        }
        "doc_class" => {
            println!(
                "Changed {} from {} to {}",
                field_name, &config.document_class, &field
            );
            config.document_class = field
        }
        "add_package" => {
            println!("Added package {}", &field);
            let s: String = field;
            config.packages.push(s);
        }
        "rm_package" => {
            println!("Removed package {}", &field);
            let mut v = Vec::new();
            for i in &config.packages {
                if i.as_str() != field.as_str() {
                    v.push(i.to_string());
                }
            }
            config.packages = v;
        }
        "only_files" => {
            println!("Changing only-files structure to {}", &field);
            let field = match field.as_str() {
                "true" => true,
                "false" => false,
                _ => false,
            };
            config.only_files = Some(field);
        }
        _ => println!("Nothing to do"),
    }
}
/// Runs the edit command for TexCreate by changing any field specified
pub async fn edit(
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
    only_files: Option<String>,
) -> TexCreateResult<()> {
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
    edit_item(&mut config, &only_files, "only_files", None);
    let mut file = File::create("config.toml").await?;
    file.write_all(config.to_string().unwrap().as_bytes())
        .await?;

    Ok(())
}
