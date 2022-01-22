mod list;
use std::borrow::BorrowMut;
use std::path::Path;
use list::list;
use structopt::StructOpt;
use texcreate_lib::routes::create;
use texcreate_lib::Config::{
    config::Config, config_multi::Config_Multi, Template
};
use texcreate_lib::Web::web::texweb;
use texcreate_lib::Templates::book::create as mkcreate;
use open::that;
use texcreate_lib::{Document, Project, tc_html};
use toml::to_string_pretty;
use std::io::stdin;

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
    #[structopt(about = "Quickly change config.toml parameters")]
    Edit{
        #[structopt(short="p", long="proj")]
        proj: Option<String>,
        #[structopt(short ="m", long="mode")]
        mode: String,
        #[structopt(long="author")]
        author: Option<String>,
        #[structopt(long="title")]
        title: Option<String>,
        #[structopt(long="date")]
        date: Option<String>,
        #[structopt(long="rename")]
        rename: Option<String>,
        #[structopt(long="template")]
        template: Option<String>,
        #[structopt(long="paper-size")]
        paper_size: Option<String>,
        #[structopt(long="font-size")]
        font_size: Option<u8>,
        #[structopt(long="doc-class")]
        doc_class: Option<String>,
        #[structopt(long="add-package")]
        add_package: Option<String>,
        #[structopt(long="rm-package")]
        rm_package: Option<String>,
    }
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cli = CLI::from_args();
    // Match CLI subcommands 
    match cli {
        CLI::Doc => {
            println!("Opening {}", TEXDOC);
            that(TEXDOC).unwrap();
        },
        CLI::Web => {
            let tc = Path::new("texcreate.html");
            let tc_file = Path::new("tc_files");

            if !tc.exists(){
                std::fs::File::create(&tc).unwrap();
                std::fs::write(&tc, tc_html.as_bytes()).unwrap();
            }
            if !tc_file.exists(){
                std::fs::create_dir(&tc_file).unwrap();
            }
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
                println!("Would you like to use the default config.toml? (y/n): ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                input = input.trim().to_string();
                if input == "y" || input == "yes"{
                    println!("Initializing single-mode config.toml...");
                    Config::init(None)?;
                } else {
                    println!("Initializing custom config.toml...");
                    // Ask for input for each field
                    let mut author = String::new();
                    let mut title = String::new();
                    let mut date = String::new();
                    let mut proj_name = String::new();
                    let mut template = String::new();
                    let mut paper_size = String::new();
                    let mut font_size = String::new();
                    let mut doc_class = String::new();

                    println!("// Enter author name:");
                    stdin().read_line(&mut author)?;
                    author = author.trim().to_string();
                    println!("// Enter project title:");
                    stdin().read_line(&mut title)?;
                    title = title.trim().to_string();
                    println!("// Enter date:");
                    stdin().read_line(&mut date)?;
                    date = date.trim().to_string();
                    println!("// Enter project name:");
                    stdin().read_line(&mut proj_name)?;
                    proj_name = proj_name.trim().to_string();
                    list();
                    println!("// Enter template:");
                    stdin().read_line(&mut template)?;
                    template = template.trim().to_string();
                    println!("// Enter paper size:");
                    stdin().read_line(&mut paper_size)?;
                    paper_size = paper_size.trim().to_string();
                    println!("// Enter font size:");
                    stdin().read_line(&mut font_size)?;
                    let fs: u8 = font_size.trim().parse().unwrap();
                    println!("// Enter document class:");
                    stdin().read_line(&mut doc_class)?;
                    doc_class = doc_class.trim().to_string();

                    let doc = Document::new(paper_size, fs, doc_class, vec![]);
                    let proj = Project::new(author, title, date, template, proj_name);
                    let conf = Config::new(proj, doc);
                    Config::init(Some(conf))?;
                }
            }
        },
        CLI::List => list(),
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
                        .await.expect("Failed to create book"),
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
            rm_package
        } => {
            // First check mode
            if mode.as_str() == "single"{
                // Since single has only one project,
                // we assume we edit this project
                let mut conf = Config::config(&None);
                match &author {
                    Some(s) => {
                        println!("Changing author from {} to {}", &conf.Project.author, s);
                        conf.Project.author = s.to_string();
                    }
                    None => println!("No changes to author field")
                }
                match &title {
                    Some(s) => {
                        println!("Changing title from {} to {}", &conf.Project.title, s);
                        conf.Project.title = s.to_string();
                    }
                    None => println!("No changes to title field")
                }
                match &date {
                    Some(s) => {
                        println!("Changing date from {} to {}", &conf.Project.date, s);
                        conf.Project.date = s.to_string();
                    }
                    None => println!("No changes to date field")
                }
                match &rename {
                    Some(s) => {
                        println!("Changing project name from {} to {}",
                                 &conf.Project.project_name, s);
                        conf.Project.project_name = s.to_string();
                    }
                    None => println!("No changes to project_name field")
                }
                match &template {
                    Some(s) => {
                        println!("Changing template from {} to {}",
                                 &conf.Project.template, s);
                        conf.Project.template = s.to_string();
                    }
                    None => println!("No changes to template field")
                }
                match &paper_size {
                    Some(s) => {
                        println!("Changing paper size from {} to {}",
                                 &conf.Document.paper_size, s);
                        conf.Document.paper_size = s.to_string();
                    }
                    None => println!("No changes to paper_size field")
                }
                match &font_size {
                    Some(s) => {
                        println!("Changing font size from {} to {}",
                                 &conf.Document.paper_size, s);
                        conf.Document.font_size = s.to_owned();
                    }
                    None => println!("No changes to font_size field")
                }
                match &doc_class {
                    Some(s) => {
                        println!("Changing document class from {} to {}",
                                 &conf.Document.document_class, s);
                        conf.Document.document_class = s.to_string();
                    }
                    None => println!("No changes to document class field")
                }
                match &add_package {
                    Some(s) => {
                        println!("Adding package {}", s);
                        conf.Document.packages.push(s.to_string());
                    }
                    None => println!("No packages added")
                }
                match &rm_package{
                    Some(s) => {
                        println!("Removing package {}", s);
                        let len = &conf.Document.packages.len();
                        for i in 0..*len{
                            if &conf.Document.packages[i] == s{
                                let p1 = &conf.Document.packages[0..i];
                                let p2 = &conf.Document.packages[i+1..*len];
                                let mut p = Vec::new();
                                for i in p1{
                                    p.push(i.to_string());
                                }
                                for i in p2{
                                    p.push(i.to_string());
                                }
                                conf.Document.packages = p;
                            }
                        }
                    }
                    None => println!("No packages removed")
                }
                let toml_s = to_string_pretty(&conf).unwrap();
                std::fs::write("config.toml", toml_s)?;
            }
            else {
                let mut conf_m = Config_Multi::config(&None);
                // Find the right project
                let mut n = 0;
                match &proj{
                    Some(s) => {
                        let len = &conf_m.Project.len();
                        for i in 0..*len{
                            if &conf_m.Project[i].project_name == s{
                                n = i;
                            }
                        }
                    }
                    None => panic!("Need specific project name (-p <name>), have to abort!")
                }
                let mut conf_proj = conf_m.Project[n].borrow_mut();
                let mut conf_doc = conf_m.Document[n].borrow_mut();
                match &author {
                    Some(s) => {
                        println!("Changing author from {} to {}", &conf_proj.author, s);
                        conf_proj.author = s.to_owned();
                    }
                    None => println!("No changes to author field")
                }
                match &title {
                    Some(s) => {
                        println!("Changing title from {} to {}", &conf_proj.title, s);
                        conf_proj.title = s.to_owned();
                    }
                    None => println!("No changes to title field")
                }
                match &date {
                    Some(s) => {
                        println!("Changing date from {} to {}", &conf_proj.date, s);
                        conf_proj.date = s.to_string();
                    }
                    None => println!("No changes to date field")
                }
                match &rename {
                    Some(s) => {
                        println!("Changing project name from {} to {}",
                                 &conf_proj.project_name, s);
                        conf_proj.project_name = s.to_owned();
                    }
                    None => println!("No changes to project_name field")
                }
                match &template {
                    Some(s) => {
                        println!("Changing template from {} to {}",
                                 &conf_proj.template, s);
                        conf_proj.template = s.to_owned();
                    }
                    None => println!("No changes to template field")
                }
                match &paper_size {
                    Some(s) => {
                        println!("Changing paper size from {} to {}",
                                 &conf_doc.paper_size, s);
                        conf_doc.paper_size = s.to_owned();
                    }
                    None => println!("No changes to paper_size field")
                }
                match &font_size {
                    Some(s) => {
                        println!("Changing font size from {} to {}",
                                 &conf_doc.paper_size, s);
                        conf_doc.font_size = s.to_owned();
                    }
                    None => println!("No changes to font_size field")
                }
                match &doc_class {
                    Some(s) => {
                        println!("Changing document class from {} to {}",
                                 &conf_doc.document_class, s);
                        conf_doc.document_class = s.to_owned();
                    }
                    None => println!("No changes to document class field")
                }
                match &add_package {
                    Some(s) => {
                        println!("Adding package {}", s);
                        conf_doc.packages.push(s.to_string());
                    }
                    None => println!("No packages added")
                }
                match &rm_package{
                    Some(s) => {
                        println!("Removing package {}", s);
                        let len = &conf_doc.packages.len();
                        for i in 0..*len{
                            if &conf_doc.packages[i] == s{
                                let p1 = &conf_doc.packages[0..i];
                                let p2 = &conf_doc.packages[i+1..*len];
                                let mut p = Vec::new();
                                for i in p1{
                                    p.push(i.to_string());
                                }
                                for i in p2{
                                    p.push(i.to_string());
                                }
                                conf_doc.packages = p;
                            }
                        }
                    }
                    None => println!("No packages removed")
                }
                conf_m.Project[n] = conf_proj.clone();
                conf_m.Document[n] = conf_doc.clone();
                let toml_s = to_string_pretty(&conf_m).unwrap();
                std::fs::write("config.toml", toml_s).unwrap();
            }
        }
    }
    Ok(())
}
