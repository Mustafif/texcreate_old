use std::fs;
use crate::BASIC;
pub fn create_basic(name: &str, path: &str) {
    let basic = load_basic();
    fs::create_dir(format!("{}/{}", path, name));
    let file_path = format!("{}/{}/{}.tex", path, name, name);
    fs::write(file_path, basic).expect("Unable to write file");
    
}

fn load_basic()-> String{
    let s = fs::read_to_string(BASIC).expect("Unable to read file");
    s
}

