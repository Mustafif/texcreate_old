mod create;
use create::routes::*;
use create::config::Config;
use create::config::Template;
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
                        ("Basic", Some(path)) => create(&name, &path, BASIC),
                        ("Math", Some(path)) => create(&name, &path, MATH),
                        ("Basic", None) => create(&name, ".", BASIC),
                        ("Math", None) => create(&name, ".", MATH),
                        ("Theatre", Some(path)) => create(&name, &path, THEATRE),
                        ("Theatre", None) => create(&name, ".", THEATRE),
                        (_, _) => println!("Please specify a template"),
                    }
                },
        CLI::List => Template::list(),
        CLI::Import{file} => {
            let conf = Config::config(&file);
            match conf.from_template(){
                Template::Basic => {
                    create(&conf.Project.project_name, ".", BASIC);
                    conf.adjust(&format!("./{}/{}.tex", conf.Project.project_name, conf.Project.project_name))
                },
                Template::Math => {
                    create(&conf.Project.project_name, ".",MATH);
                    conf.adjust(&format!("./{}/{}.tex", conf.Project.project_name, conf.Project.project_name))
                },
                Template::Theatre => {
                    create(&conf.Project.project_name, ".", THEATRE);
                    conf.adjust(&format!("./{}/{}.tex", conf.Project.project_name, conf.Project.project_name))
                },
                _ => println!("Error in {}, make sure template is valid", &file)
            }
    }
}
}
