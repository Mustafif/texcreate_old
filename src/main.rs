mod list;
use list::List;
use structopt::StructOpt;
use texcreate_lib::routes::create;
use texcreate_lib::Config::{
    config::Config, config_multi::Config_Multi, Template
};
use texcreate_lib::Web::web::texweb;
use texcreate_lib::Templates::book::create as mkcreate;
use open::that;
use texcreate_lib::from_template;

const TEXDOC: &str = "http://texcreate.mkproj.com/";
macro_rules! import_temp {
    ($c:expr, $x: expr) => {
        create(&$c.Project.project_name, &None , $x);
        match $c.adjust(&format!(
            "./{}/{}.tex",
            $c.Project.project_name, $c.Project.project_name
        )){
            Ok(a) => a,
            Err(e) => {
                eprintln!("{}", e)
            }
        };

        $c.add_packages(&format!("./{}/structure.tex", $c.Project.project_name));
    };
}
#[derive(StructOpt, Debug)]
#[structopt(
    name = "texcreate",
    about = "Create LaTeX projects using prebuilt templates"
)]
/// All TexCreate Subcommands
enum CLI {
    /// Update TexCreate
    #[structopt(name = "update", about = "Updates to latest version")]
    Update,
    #[structopt(about = "Initialize a config.toml file")]
    /// Initialize with `texcreate init`
    Init{
        /// Initialize with multi-mode
        #[structopt(short = "m", long = "mode")]
        mode: Option<String>,
    },
    #[structopt(about = "Create a LaTeX Project with a specified name & template")]
    /// Create project with `texcreate create -t <template> -n <name>`
    Create {
        #[structopt(short = "t", long = "template", help = "Template to use")]
        /// Template to use
        template: String,
        #[structopt(short = "n", long = "name", help = "Name of the project")]
        /// Project name
        name: String,
        #[structopt(
            short = "d",
            long = "directory",
            help = "Directory to create the project in"
        )]
        /// Optional output directory
        path: Option<String>,
    },
    #[structopt(about = "Lists all the available templates")]
    /// List all available templates `texcreate list`
    List,
    #[structopt(about = "Build a project using a config.toml file")]
    /// Create project with a config.toml file `texcreate import -f config.toml`
    Build {
        #[structopt(short, long)]
        file: Option<String>,
        #[structopt(short, long)]
        mode: Option<String>,
    },
    #[structopt(about="Opens the TexCreate documentation")]
    Doc, 
    #[structopt(about="Opens the TexCreate web version")]
    Web,
}
#[tokio::main]
async fn main() {
    let cli = CLI::from_args();
    // Match CLI subcommands 
    match cli {
        CLI::Doc => {
            println!("Opening {}", TEXDOC);
            that(TEXDOC).unwrap();
        },
        CLI::Web => {
            texweb().launch().await.unwrap();
        }
        CLI::Update => {
            let _ = std::process::Command::new("cargo")
                .arg("install")
                .arg("texcreate")
                .spawn()
                .expect("Failed to install latest version");
        }
        CLI::Init {mode} => {
            let mode = match &mode{
                Some(m) => m,
                None => "single"
            };

            if mode == "multi"{
                println!("Initializing multi-mode config.toml...");
                Config_Multi::init();
            } else {
                println!("Creating config.toml...");
                Config::init();
            }
        },
        CLI::Create {
            template,
            name,
            path,
        } => match (template.as_str(), path) {
            (temp, path) => create(&name, &path, temp),
        },
        CLI::List => List::list("list.json"),
        CLI::Build { file, mode } => {
            let mode = match &mode{
                Some(m) => m,
                None => "single"
            };
            if mode == "single"{
                let conf = Config::config(&file);
                match conf.from_template() {
                    Template::Basic => {
                        import_temp!(conf, "Basic");
                    }
                    Template::Math => {
                        import_temp!(conf, "Math");
                    }
                    Template::Theatre => {
                        import_temp!(conf, "Theatre");
                    }
                    Template::Code => {
                        import_temp!(conf, "Code");
                    }
                    Template::Novel =>{
                        import_temp!(conf, "Novel");
                    }
                    Template::Beamer => {
                        import_temp!(conf, "Beamer");
                    }
                    Template::Lachaise => {
                        import_temp!(conf, "Lachaise");
                    }
                    Template::Book => mkcreate(
                        &conf.Project.project_name,
                        &conf.Project.title,
                        &conf.Project.author,
                    )
                        .await
                        .expect("Couldn't make Book Project"),
                }
            }
                //Mutli mode
            else {
                let conf = Config_Multi::config(&file);
                match conf.adjust(&None){
                    Ok(a) => a,
                    Err(e) => eprintln!("{}", e)
                };
            }

        }
    }
}
