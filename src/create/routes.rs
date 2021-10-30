use super::template::*;
use std::io::Write;

pub fn basic() -> (String, String) {
    (BASIC_MAIN.to_string(), BASIC_STRUCTURE.to_string())
}
pub fn math() -> (String, String) {
    (MATH_MAIN.to_string(), MATH_STRUCTURE.to_string())
}

pub fn theatre() -> (String, String) {
    (THEATRE_MAIN.to_string(), THEATRE_STRUCTURE.to_string())
}

pub fn load(template: &str) -> (String, String) {
    match template {
        "Basic" => basic(),
        "Math" => math(),
        "Theatre" => theatre(),
        _ => basic(),
    }
}
pub fn create(name: &str, file_path: &str, template: &str) {
    let (main, structure) = load(template);
    std::fs::create_dir(format!("{}/{}", file_path, name)).unwrap();
    let mut main_file =
        std::fs::File::create(format!("{}/{}/{}.tex", file_path, name, name)).unwrap();
    main_file.write_all(main.as_bytes()).unwrap();
    let mut structure_file =
        std::fs::File::create(format!("{}/{}/structure.tex", file_path, name)).unwrap();
    structure_file.write_all(structure.as_bytes()).unwrap();
}
