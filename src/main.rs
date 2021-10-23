mod create;
pub const BASIC: &str = "Templates/Basic/main.tex";
pub const MATH_MAIN: &str = "Templates/Math/main.tex";
pub const MATH_STRUCTURE: &str = "Templates/Math/structure.tex";
pub const THEATRE: &str = "Templates/Theatre/main.tex";

use create::basic::create_basic;
use create::math::create_math;
use create::theatre::create_theatre;
use create::config::Config;
use create::config::{Project, Template};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "texcreate", 
    about = "Create LaTeX projects using prebuilt templates"
)]
enum CLI{
    #[structopt(about= "Initialize a config.toml file")]
    Init, 
    #[structopt(about = "Create a LaTeX Project with a specified name & template")]
    Create{
        #[structopt(short = "t", long = "template", help = "Template to use")]
        template: String, 
        #[structopt(short = "n", long = "name", help = "Name of the project")]
        name: String,
        #[structopt(short = "d", long = "directory", help = "Directory to create the project in")]
        path: Option<String>,
    },
    #[structopt(about = "Lists all the available templates")]
    List,
    #[structopt(about = "Import a config.toml to create project")]
    Import{
        #[structopt(short, long)]
        file: String
    }
}


fn main() {
    let CLI = CLI::from_args();
    match CLI {
        CLI::Init => Config::init(),
        CLI::Create{template, name, path} => {
            match (template.as_str(), path){
                        ("Basic", Some(path)) => create_basic(&name, &path),
                        ("Math", Some(path)) => create_math(&name, &path),
                        ("Theatre", Some(path)) => create_theatre(&name, &path),
                        ("Math", None) => create_math(&name, "."),
                        ("Basic", None) => create_basic(&name, "."),
                        _ => println!("Template not found")
                    }
                },
        CLI::List => Template::list(),
        CLI::Import{file} => {
            let conf = Config::config(&file);
            match conf.from_template(){
                Template::Basic => {
                    create_basic(&conf.Project.project_name, ".");
                    conf.adjust(&format!("./{}/{}.tex", conf.Project.project_name, conf.Project.project_name))
                },
                Template::Math => {
                    create_math(&conf.Project.project_name, ".");
                    conf.adjust(&format!("./{}/{}.tex", conf.Project.project_name, conf.Project.project_name))
                },
                Template::Theatre => {
                    create_theatre(&conf.Project.project_name, ".");
                    conf.adjust(&format!("./{}/{}.tex", conf.Project.project_name, conf.Project.project_name))
                },
                _ => println!("Error in {}, make sure template is valid", &file)
            }
    }
}
}
