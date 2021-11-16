use structopt::StructOpt;
use texcreate_lib::routes::create;
use texcreate_lib::Config::{config::Config, List, Template};
use texcreate_lib::Templates::book::create as mkcreate;

macro_rules! import_temp {
    ($c:expr, $x: expr) => {
        create(&$c.Project.project_name, &None , $x);
        $c.adjust(&format!(
            "./{}/{}.tex",
            $c.Project.project_name, $c.Project.project_name
        ));
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
    #[structopt(name = "update", about = "Updates to latest version")]
    Update,
    #[structopt(about = "Initialize a config.toml file")]
    /// Initialize with `texcreate init`
    Init,
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
    #[structopt(about = "Import a config.toml to create project")]
    /// Create project with a config.toml file `texcreate import -f config.toml`
    Import {
        #[structopt(short, long)]
        file: Option<String>,
    },
}
#[tokio::main]
async fn main() {
    let cli = CLI::from_args();
    match cli {
        CLI::Update => {
            let _ = std::process::Command::new("cargo")
                .arg("install")
                .arg("texcreate")
                .spawn()
                .expect("Failed to install latest version");
        }
        CLI::Init => Config::init(),
        CLI::Create {
            template,
            name,
            path,
        } => match (template.as_str(), path) {
            (temp, path) => create(temp, &path, temp),
        },
        CLI::List => List::list("list.json"),
        CLI::Import { file } => {
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
                Template::Book => mkcreate(
                    &conf.Project.project_name,
                    &conf.Project.title,
                    &conf.Project.author,
                )
                .await
                .unwrap(),
            }
        }
    }
}
