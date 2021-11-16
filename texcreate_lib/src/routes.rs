use crate::Templates::*;
use std::io::Write;

fn const_conv(main: &str, structure: &str)-> (String, String) {
    (main.to_string(), structure.to_string())
}

fn load(template: &str) -> (String, String) {
    match template {
        "Basic" => const_conv(basic::BASIC_MAIN, basic::BASIC_STRUCTURE),
        "Code" => const_conv(code::CODE_MAIN, code::CODE_STRUCTURE),
        "Math" => const_conv(math::MATH_MAIN, math::MATH_STRUCTURE),
        "Theatre" => const_conv(theatre::THEATRE_MAIN, theatre::THEATRE_STRUCTURE),
        _ => panic!("Unknown template: {}", template),
    }
}

pub fn create(name: &str, file_path: &Option<String>, template: &str) {
    // Sanity check for file_path
    if let file_path = Some(path){
        file_path = Some(path);
    } else {
        file_path = "."
    }
    let (main, structure) = load(template);
    std::fs::create_dir(format!("{}/{}", file_path, name)).unwrap();
    let mut main_file =
        std::fs::File::create(format!("{}/{}/{}.tex", file_path, name, name)).unwrap();
    main_file.write_all(main.as_bytes()).unwrap();
    let mut structure_file =
        std::fs::File::create(format!("{}/{}/structure.tex", file_path, name)).unwrap();
    structure_file.write_all(structure.as_bytes()).unwrap();
}