use std::fs;
use std::io::Write;
use crate::{MATH_MAIN, MATH_STRUCTURE};


pub fn create_math(name: &str, file_path: &str) {
    let (main, structure) = load_math();
    fs::create_dir(format!("{}/{}", file_path, name)).unwrap();
    let mut file = fs::File::create(format!("{}/{}/{}.tex", file_path, name, name)).unwrap();
    file.write_all(main.as_bytes()).unwrap();
    let mut structure_file = fs::File::create(format!("{}/{}/{}_structure.tex", file_path,name,  name)).unwrap();
    structure_file.write_all(structure.as_bytes()).unwrap();

}

fn load_math()-> (String, String){
    let s = fs::read_to_string(MATH_MAIN).expect("Unable to read file");
    let s2 = fs::read_to_string(MATH_STRUCTURE).expect("Unable to read file");
    (s, s2)
}