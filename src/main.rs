use async_std::fs::{read_to_string, File};
use async_std::io::prelude::*;
use config::*;
use structopt::StructOpt;
use texc_config as config;
use texc_utils as utils;
use texcreate_lib::Errors;
use tokio::io::AsyncWriteExt;
use toml::from_str;
use utils::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "texcreate",
    about = "Create LaTeX projects using prebuilt templates"
)]
enum CLI {
    /// Update TexCreate
    #[structopt(name = "update", about = "Updates to latest version")]
    Update,
    #[structopt(about = "Initialize a config.toml file")]
    /// Initialize with `texcreate init`
    Init {
        /// Initialize with multi-mode
        #[structopt(short = "m", long = "mode")]
        mode: Option<String>,
    },
    #[structopt(about = "Lists all the available templates")]
    /// List all available templates `texcreate list`
    List,
    #[structopt(about = "Build a project using a config.toml file")]
    /// Create project with a config.toml file `texcreate build`
    Build {
        #[structopt(short, long)]
        file: Option<String>,
        #[structopt(short, long)]
        mode: Option<String>,
    },
    #[structopt(about = "Opens the TexCreate documentation")]
    Doc,
    #[structopt(about = "Opens the TexCreate web version")]
    Web,
    #[structopt(about = "Quickly change config.toml parameters")]
    Edit {
        #[structopt(short = "p", long = "proj")]
        proj: Option<String>,
        #[structopt(short = "m", long = "mode")]
        mode: String,
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
    },
    #[structopt(about = "Migrate single mode config.toml from v1 to v2")]
    Migrate,
    #[structopt(about = "Compile TexCreate Project with set TeX Compiler")]
    Compile,
}

#[tokio::main]
async fn main() -> TexCreateResult<()> {
    let cli: CLI = CLI::from_args();
    match cli {
        CLI::Update => {
            update()?;
        }
        CLI::Init { mode } => {
            init(mode).await?;
        }
        CLI::List => {
            list();
        }
        CLI::Build { file, mode } => {
            let file = file.unwrap_or_else(|| "config.toml".to_string());
            let mode = mode.unwrap_or_else(|| "single".to_string());

            if &mode == "mutli" {
                let multi = from_str_multi(read_to_string(&file).await?).unwrap();
                for i in multi {
                    i.build().await?;
                }
            } else {
                let config = Config::from_string(read_to_string(&file).await?).unwrap();
                config.build().await?;
            }
        }
        CLI::Doc => {
            println!("texcreate.mkproj.com coming v2.0.0-beta.3")
        }
        CLI::Web => {
            println!("Coming V2.0.0-beta.2")
        }
        CLI::Edit {
            proj,
            mode,
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
        } => {
            edit(proj,
                 mode,
                 author,
                 title,
                 date,
                 rename,
                 template,
                 paper_size,
                 font_size,
                 doc_class,
                 add_package,
                 rm_package).await?;
        }
        CLI::Migrate => {
            let s = Config::migrate().unwrap();
            let mut file = File::create("config.toml").await?;
            file.write_all(s.as_bytes()).await?;
            println!("Sucessfully migrated config.toml to v2")
        }
        CLI::Compile => {
            let texc: TexcToml = from_str(&read_to_string("texcreate.toml").await?).unwrap();
            texc.compile().await?;
        }
    }

    Ok(())
}
