pub const BASIC: &str = "Templates/Basic/";
pub const MATH: &str = "Templates/Math/";
pub const THEATRE: &str = "Templates/Theatre/";
use std::io::Write;

pub fn load(path: &str) -> (String, String){
    let main = std::fs::read_to_string(format!("{}main.tex", path)).unwrap();
    let structure = std::fs::read_to_string(format!("{}structure.tex", path)).unwrap();
    (main, structure)
}

pub fn create(name: &str, file_path: &str, template: &str) {
    let (main, structure) = load(template);
    std::fs::create_dir(format!("{}/{}", file_path, name)).unwrap();
    let mut main_file = std::fs::File::create(format!("{}/{}/{}.tex", file_path, name, name)).unwrap();
    main_file.write_all(main.as_bytes()).unwrap();
    let mut structure_file = std::fs::File::create(format!("{}/{}/structure.tex", file_path,name)).unwrap();
    structure_file.write_all(structure.as_bytes()).unwrap();
}