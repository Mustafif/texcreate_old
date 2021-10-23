use std::fs;
use std::io::{Write, Read};
use crate::THEATRE;

pub fn create_theatre(name: &str, file_path: &str){
    let main = load_theatre();
    fs::create_dir(format!("{}/{}", file_path, name)).unwrap();
    let mut file = fs::File::create(format!("{}/{}/{}.tex", file_path, name, name)).unwrap();
    file.write_all(main.as_bytes()).unwrap();
}

pub fn load_theatre()-> String{
    let mut file = fs::File::open(THEATRE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}