//! TexCreate Config Library <br>
//! This library contains all code related to `config.toml` <br>
//! Developer: Mustafif Khan <br>
//! License: MIT & GPLv2

use std::fmt::format;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

use async_std::fs::*;
use async_std::io::prelude::*;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use texcreate_lib::config::{Config as LegacyConfig, Document, Project};
use texcreate_lib::Errors;
use toml::{from_str, to_string_pretty};
use zip::CompressionMethod::Stored;
use zip::read::ZipFile;
use zip::write::{FileOptions, ZipWriter};

pub use error::*;
pub use extra::*;
use tex_rs::Latex;
use texc_latex::templates::*;

use crate::TexCreateError::Invalid;

/// Contains all error handling using `failure` crate
pub mod error;
/// Contains all code related to the `README.md` & `texcreate.toml`
pub mod extra;

type F = std::fs::File;


/// The Config struct contains all necessary metadata information
/// - Author: The author of the project
/// - Title: The title of the document
/// - Date: The date for the document
/// - Project Name: The name for the project
/// - Template: The prebuilt template to use
/// - Paper-size: The paper-size for the document
/// - Document Class: The document class of the document
/// - Font size: The font size for the document
/// - Packages: Additional packages to add
/// - Language: __Not supported yet__
/// - Only Files: Whether structure is project or files _(default: false)_
#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct Config {
    pub author: String,
    pub title: String,
    pub date: String,
    pub project_name: String,
    pub template: String,
    pub paper_size: String,
    pub document_class: String,
    pub font_size: u8,
    pub packages: Vec<String>,
    pub language: Option<String>,
    pub only_files: Option<bool>,
}

impl Default for Config {
    /// Default settings for Config
    fn default() -> Self {
        Self {
            author: "Author".to_string(),
            title: "Title".to_string(),
            date: "Date".to_string(),
            project_name: "Project".to_string(),
            template: "Basic".to_string(),
            paper_size: "letterpaper".to_string(),
            document_class: "article".to_string(),
            font_size: 11,
            packages: vec![],
            language: None,
            only_files: None,
        }
    }
}

impl Config {
    /// Creates a new Config
    pub fn new(
        author: &str,
        title: &str,
        date: &str,
        project_name: &str,
        template: &str,
        paper_size: &str,
        font_size: u8,
        packages: Vec<&str>,
        language: Option<&str>,
        only_files: Option<bool>,
        document_class: &str,
    ) -> Self {
        Self {
            author: author.to_string(),
            title: title.to_string(),
            date: date.to_string(),
            project_name: project_name.to_string(),
            template: template.to_string(),
            paper_size: paper_size.to_string(),
            document_class: document_class.to_string(),
            font_size,
            packages: packages.iter().map(|x| x.to_string()).collect(),
            language: language.map(|x| x.to_string()),
            only_files,
        }
    }
    /// Using `toml::to_string_pretty()`, turns Config into toml string for `config.toml`
    pub fn to_string(&self) -> Result<String, toml::ser::Error> {
        Ok(to_string_pretty(self)?)
    }
    /// From `config.toml` string, turns into Config
    pub fn from_string(s: String) -> Result<Self, toml::de::Error> {
        Ok(from_str(&s)?)
    }
    /// Turns a v1 `config.toml` into v2 Config toml string
    pub fn migrate() -> Result<String, toml::ser::Error> {
        let legacy = LegacyConfig::config(&None);
        let config = Config::new(
            &legacy.Project.author,
            &legacy.Project.title,
            &legacy.Project.date,
            &legacy.Project.project_name,
            &legacy.Project.template,
            &legacy.Document.paper_size,
            legacy.Document.font_size,
            legacy
                .Document
                .packages
                .iter()
                .map(|x| x.as_str())
                .collect(),
            None,
            None,
            &legacy.Document.document_class,
        );
        config.to_string()
    }

    /// Grabs the `tex_rs::Latex` template from `&self.template` by matching the string
    pub fn template(&self) -> TexCreateResult<Latex> {
        let f = match &*self.template {
            "Basic" => basic(
                self.clone().font_size,
                &self.paper_size,
                &self.document_class,
                &self.author,
                &self.title,
                &self.date,
                &self.packages
            ),
            "Code" => code(
                self.clone().font_size,
                &self.paper_size,
                &self.document_class,
                &self.author,
                &self.title,
                &self.date,
                &self.packages
            ),
            "Novel" => novel(
                self.clone().font_size,
                &self.paper_size,
                &self.document_class,
                &self.author,
                &self.title,
                &self.date,
                &self.packages
            ),
            "Theatre" => theatre(
                self.clone().font_size,
                &self.paper_size,
                &self.document_class,
                &self.author,
                &self.title,
                &self.date,
                &self.packages
            ),
            "Lachaise" => lachaise(
                self.clone().font_size,
                &self.paper_size,
                &self.document_class,
                &self.author,
                &self.title,
                &self.date,
                &self.packages
            ),
            "Beamer" => beamer(
                self.clone().font_size,
                &self.paper_size,
                &self.document_class,
                &self.author,
                &self.title,
                &self.date,
                &self.packages
            ),
            _ => return Err(TexCreateError::InvalidTemplate(self.template.clone())),
        };
        Ok(f)
    }
    /// Builds project depending on `Config.only_files`
    pub async fn build(&self, force: Option<bool>) -> TexCreateResult<()> {
        let force = match force{
            Some(a) => a,
            None => false
        };

        if !force{
            println!("Checking for any errors...");
            match check_errors(&self){
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    return Err(exit(1))
                }
            };
        }

        println!("Loading template: {}", &self.template);
        if &self.template == "Book"{
            book(&self.project_name, &self.title, &self.author).await.unwrap();
            return Ok(());
        }
        let latex = self.template()?;
        println!("Creating project: {}", &self.project_name);
        let path = Path::new(&self.project_name);
        // Project/
        create_dir(&path).await?;
        if self.only_files == None || self.only_files.unwrap() == false {
            // README.md
            let readme_path = path.to_path_buf().join("README.md");
            let mut readme = File::create(readme_path).await?;
            readme
                .write_all(readme_make(&self.project_name).as_bytes())
                .await?;
            // texcreate.toml
            write_toml(path.to_path_buf(), &self.project_name).await?;
            // out/
            let out = path.to_path_buf().join("out");
            create_dir(&out).await?;
            // src/
            let src = path.to_path_buf().join("src");
            create_dir(&src).await?;
            println!("Writing Latex Template in {}", &src.to_str().unwrap());
            latex
                .split_write(
                    src.clone().join(&format!("{}.tex", &self.project_name)),
                    src.clone().join("structure.tex"),
                )
                .await?;
        } else {
            let path = path.to_path_buf();
            latex
                .split_write(
                    path.clone().join(&format!("{}.tex", &self.project_name)),
                    path.clone().join("structure.tex"),
                )
                .await?;
        }
        Ok(())
    }
    /// Zips a project structure
    pub async fn zip_proj(&self, path: PathBuf) -> TexCreateResult<()> {
        let mut zip = ZipWriter::new(
            F::create(format!(
                "{}/{}.zip",
                path.to_str().unwrap(),
                &self.project_name
            ))
            .unwrap(),
        );
        let options = FileOptions::default().compression_method(Stored);

        // README.md
        let readme_path = path.join("README.md");
        let _ = F::create(&readme_path).unwrap();
        zip.start_file(readme_path.to_str().unwrap(), options)
            .unwrap();
        zip.write_all(README.as_bytes()).unwrap();
        // texcreate.toml
        let toml_path = path.join("texcreate.toml");
        let _ = F::create(&toml_path).unwrap();
        zip.start_file(toml_path.to_str().unwrap(), options)
            .unwrap();
        let mut tex_toml = TexcToml::default();
        tex_toml.project_name = self.clone().project_name;
        zip.write_all(to_string_pretty(&tex_toml).unwrap().as_bytes())
            .unwrap();
        // out/
        let out_path = path.join("out");
        create_dir(&out_path).await?;
        zip.add_directory(out_path.to_str().unwrap(), options).unwrap();
        // src/
        let src = path.join("src");
        create_dir(&src).await?;

        // src/*.tex
        let temp_str = self.template()?.split_string();

        let main = src.join(format!("{}.tex", &self.project_name));
        let _ = F::create(&main).unwrap();

        let structure = src.join("structure.tex");
        let _ = F::create(&structure).unwrap();

        zip.start_file(main.to_str().unwrap(), options).unwrap();
        zip.write_all(&temp_str.0.as_bytes()).unwrap();

        zip.start_file(structure.to_str().unwrap(), options).unwrap();
        zip.write_all(&temp_str.1.as_bytes()).unwrap();

        zip.finish().unwrap();

        // Cleaning step
        remove_file(&readme_path).await?;
        remove_file(&toml_path).await?;
        remove_dir(&out_path).await?;
        remove_dir_all(&src).await?;

        Ok(())
    }
    /// Zips a only files structure
    pub async fn zip_files(&self, path: PathBuf) -> TexCreateResult<()> {
        let mut zip = ZipWriter::new(
            F::create(format!(
                "{}/{}.zip",
                path.to_str().unwrap(),
                &self.project_name
            ))
            .unwrap(),
        );
        let options = FileOptions::default().compression_method(Stored);

        let main = format!("{}/{}.tex", path.to_str().unwrap(), &self.project_name);
        let structure = path.join("structure.tex");
        let _ = File::create(&main);
        let _ = File::create(&structure);
        let temp_str = self.template()?.split_string();

        zip.start_file(&main, options).unwrap();
        zip.write_all(&temp_str.0.as_bytes()).unwrap();

        zip.start_file(structure.to_str().unwrap(), options)
            .unwrap();
        zip.write_all(&temp_str.1.as_bytes()).unwrap();

        zip.finish().unwrap();

        // Cleaning step
        remove_file(&main).await?;
        remove_file(&structure).await?;

        Ok(())
    }
}
