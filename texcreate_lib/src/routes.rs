use crate::Templates::*;
use std::io::Write;
use std::path::Path;
use std::fs::create_dir;
use log::warn;
fn const_conv(main: &str, structure: &str)-> (String, String) {
    (main.to_string(), structure.to_string())
}

fn load(template: &str) -> (String, String) {
    match template {
        "Basic" => const_conv(basic::BASIC_MAIN, basic::BASIC_STRUCTURE),
        "Code" => const_conv(code::CODE_MAIN, code::CODE_STRUCTURE),
        "Math" => const_conv(math::MATH_MAIN, math::MATH_STRUCTURE),
        "Theatre" => const_conv(theatre::THEATRE_MAIN, theatre::THEATRE_STRUCTURE),
        "Novel" => const_conv(novel::NOVEL_MAIN, novel::NOVEL_STRUCTURE),
        "Beamer" => const_conv(beamer::BEAMER_MAIN, beamer::BEAMER_STRUCTURE),
        _ => panic!("Unknown template: {}", template),
    }
}

pub fn create(name: &str, file_path: &Option<String>, template: &str) {
    // Sanity check for file_path
    let file_path = match file_path{
        Some(path) => path,
        None => "."
    };
    let (main, structure) = load(template);
    let path = format!("{}/{}", file_path, name);
    let def_path = format!("{}/{}_texcreate", file_path, name);
    let mut p = String::new();
    // Check if directory exists, if it does, use def
    // else create the directory 
    if Path::new(&path).exists(){
        let f = format!("Note: {} exists, creating {} instead\n", &path, &def_path);
        println!("{}",f);
        create_dir(&def_path).expect("Error in creating directory");
        p = def_path;
    } else {
        println!("Creating {}", &path);
        create_dir(&path).expect("Error in creating directory");
        p = path;
    }
    let mut main_file =
        std::fs::File::create(format!("{}/{}.tex", &p, name)).unwrap();
    main_file.write_all(main.as_bytes()).unwrap();
    let mut structure_file =
        std::fs::File::create(format!("{}/structure.tex", &p)).unwrap();
    structure_file.write_all(structure.as_bytes()).unwrap();
}