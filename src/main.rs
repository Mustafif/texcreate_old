//! TexCreate <br>
//! A LaTeX Project Creator using prebuilt templates <br>
//! Developer: Mustafif Khan <br>
//! License: MIT & GPLv2 <br>
//! This project contains a family of libraries that contribute
//! to various areas of the project, and can be used standalone, or
//! implemented in a custom TexCreate.
//! - [tex-rs](https://docs.rs/tex-rs/)
//! - [texc-config](https://docs.rs/texc-config/)
//! - [texc-latex](https://docs.rs/texc-latex/)
//! - [texc-utils](https://docs.rs/texc-utils/)
//! - [texc-web](https://docs.rs/texc-web)


use std::path::PathBuf;

use async_std::fs::{read_to_string, File};
use async_std::io::prelude::*;
use structopt::StructOpt;
use toml::from_str;

use config::*;
use texc_config as config;
use texc_utils as utils;
use texc_web::texweb;
use utils::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "texcreate",
    about = "Create LaTeX projects using prebuilt templates"
)]
enum CLI{
    /// Update TexCreate
    #[structopt(name = "update", about = "Updates to latest version")]
    Update,
    #[structopt(about = "Initialize a config.toml file")]
    /// Initialize with `texcreate init`
    Init,
    #[structopt(about = "Lists all the available templates")]
    /// List all available templates `texcreate list`
    List,
    #[structopt(about = "Build a project using a config.toml file")]
    /// Create project with a config.toml file `texcreate build`
    Build {
        #[structopt(long)]
        file: Option<String>,
        #[structopt(long)]
        force : Option<bool>
    },
    #[structopt(about = "Opens the TexCreate documentation")]
    Doc,
    #[structopt(about = "Opens the TexCreate web version")]
    Web,
    #[structopt(about = "Quickly change config.toml parameters")]
    Edit {
        #[structopt(long = "author")]
        author: Option<String>,
        #[structopt(long = "title")]
        title: Option<String>,
        #[structopt(long = "date")]
        date: Option<String>,
        #[structopt(long = "rename")]
        rename: Option<String>,
        #[structopt(long = "template")]
        template: Option<String>,
        #[structopt(long = "paper-size")]
        paper_size: Option<String>,
        #[structopt(long = "font-size")]
        font_size: Option<u8>,
        #[structopt(long = "doc-class")]
        doc_class: Option<String>,
        #[structopt(long = "add-package")]
        add_package: Option<String>,
        #[structopt(long = "rm-package")]
        rm_package: Option<String>,
        #[structopt(long = "only-files")]
        only_files: Option<String>,
    },
    #[structopt(about = "Migrate single mode config.toml from v1 to v2")]
    Migrate,
    #[structopt(about = "Compile TexCreate Project with set TeX Compiler")]
    Compile,
    #[structopt(about = "Zip TexCreate Project instead of building")]
    Zip,
}

#[tokio::main]
async fn main() -> TexCreateResult<()> {
    let cli: CLI = CLI::from_args();
    match cli {
        CLI::Update => {
            update()?;
        }
        CLI::Init => {
            init().await?;
        }
        CLI::List => {
            list();
        }
        CLI::Build { file, force } => {
            let file = file.unwrap_or_else(|| "config.toml".to_string());

            let config = Config::from_string(read_to_string(&file).await?).unwrap();
            config.build(force).await?;
        }
        CLI::Doc => {
            match open::that("http://docs.rs/texcreate"){
                Ok(_) => println!("Opened docs successfully!"),
                Err(e) => println!("Error: {}", e)
            }
        }
        CLI::Web => texweb().launch().await.unwrap(),
        CLI::Edit {
            author,
            title,
            date,
            rename,
            template,
            paper_size,
            font_size,
            doc_class,
            add_package,
            rm_package,
            only_files,
        } => {
            edit(
                author,
                title,
                date,
                rename,
                template,
                paper_size,
                font_size,
                doc_class,
                add_package,
                rm_package,
                only_files,
            )
            .await?;
        }
        CLI::Migrate => {
            // Do note due to the complex structure of v1 multi configs
            // Migrate is only supported for single mode
            let s = Config::migrate().unwrap();
            let mut file = File::create("config.toml").await?;
            file.write_all(s.as_bytes()).await?;
            println!("Sucessfully migrated config.toml to v2")
        }
        CLI::Compile => {
            let texc: TexcToml = from_str(&read_to_string("texcreate.toml").await?).unwrap();
            texc.compile().await?;
        }
        CLI::Zip => {
            let config: Config = from_str(&read_to_string("config.toml").await?).unwrap();

            let only_files = match config.only_files.clone() {
                Some(p) => p,
                None => false,
            };

            if only_files {
                config.zip_files(&PathBuf::from("")).await?;
            } else {
                config.zip_proj(&PathBuf::from("")).await?;
            }
            println!("Successfully zipped project: {}", &config.project_name);
        }
    }

    Ok(())
}
