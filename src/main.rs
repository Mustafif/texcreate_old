mod create;
use create::config::Config;
use create::config::{List, Template};
use create::mkproj_book::create as mkcreate;
use create::routes::create;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(
    name = "texcreate",
    about = "Create LaTeX projects using prebuilt templates"
)]
/// All TexCreate Subcommands
pub enum CLI {
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
    let CLI = CLI::from_args();
    match CLI {
        CLI::Init => Config::init(),
        CLI::Create {
            template,
            name,
            path,
        } => match (template.as_str(), path) {
            ("Basic", Some(path)) => create(&name, &path, "Basic"),
            ("Math", Some(path)) => create(&name, &path, "Math"),
            ("Basic", None) => create(&name, ".", "Basic"),
            ("Math", None) => create(&name, ".", "Math"),
            ("Theatre", Some(path)) => create(&name, &path, "Theatre"),
            ("Theatre", None) => create(&name, ".", "Theatre"),
            (_, _) => println!("Please specify a template"),
        },
        CLI::List => List::list("list.json"),
        CLI::Import { file } => {
            let conf = Config::config(&file);
            match conf.from_template() {
                Template::Basic => {
                    create(&conf.Project.project_name, ".", "Basic");
                    conf.adjust(&format!(
                        "./{}/{}.tex",
                        conf.Project.project_name, conf.Project.project_name
                    ));
                    conf.add_packages(&format!("./{}/structure.tex", conf.Project.project_name));
                }
                Template::Math => {
                    create(&conf.Project.project_name, ".", "Math");
                    conf.adjust(&format!(
                        "./{}/{}.tex",
                        conf.Project.project_name, conf.Project.project_name
                    ));
                    conf.add_packages(&format!("./{}/structure.tex", conf.Project.project_name));
                }
                Template::Theatre => {
                    create(&conf.Project.project_name, ".", "Theatre");
                    conf.adjust(&format!(
                        "./{}/{}.tex",
                        conf.Project.project_name, conf.Project.project_name
                    ));
                    conf.add_packages(&format!("./{}/structure.tex", conf.Project.project_name));
                }
                Template::Book => mkcreate(
                    &conf.Project.project_name,
                    &conf.Project.title,
                    &conf.Project.author,
                )
                .await
                .unwrap(),
                _ => println!("Make sure template is valid"),
            }
        }
    }
}
