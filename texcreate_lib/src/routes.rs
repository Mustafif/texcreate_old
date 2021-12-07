use crate::Templates::*;
use std::io::Write;
use std::path::Path;
use std::fs::create_dir;
/// Constant Convert 
/// 
/// Used to have the template constants into an easy tuple 
/// 
/// # Examples
/// 
/// ```
/// use texcreate_lib::routes::const_conv;
/// use texcreate_lib::Templates::basic;
/// 
/// // Let's build a basic template
/// fn main(){
///     let basic = const_conv(Basic_MAIN, BASIC_STRUCTURE);
///     // Write main.tex 
///     let mut main = std::fs::File::create("main.tex").unwrap();
///     main.write_all(basic.0.as_bytes()).unwrap();
///     // Write structure.tex
///    let mut structure = std::fs::File::create("structure.tex").unwrap();
///    structure.write_all(basic.1.as_bytes()).unwrap();
/// }
/// ```


pub fn const_conv(main: &str, structure: &str)-> (String, String) {
    (main.to_string(), structure.to_string())
}

/// Load Template 
/// 
/// Given a template name, it will load the template and return it as a string tuple 
/// 
/// # Examples
/// 
/// ```
/// use texcreate_lib::routes::load;
/// 
/// // Recreate the basic template from `const_conv`: 
/// fn main(){
///   let basic = load("Basic");
///  // Write main.tex
///  let mut main = std::fs::File::create("main.tex").unwrap();
///  main.write_all(basic.0.as_bytes()).unwrap();
/// // Write structure.tex
/// let mut structure = std::fs::File::create("structure.tex").unwrap();
/// structure.write_all(basic.1.as_bytes()).unwrap();
/// }
/// ```



pub fn load(template: &str) -> (String, String) {
    match template {
        "Basic" => const_conv(basic::BASIC_MAIN, basic::BASIC_STRUCTURE),
        "Code" => const_conv(code::CODE_MAIN, code::CODE_STRUCTURE),
        "Math" => const_conv(math::MATH_MAIN, math::MATH_STRUCTURE),
        "Theatre" => const_conv(theatre::THEATRE_MAIN, theatre::THEATRE_STRUCTURE),
        "Novel" => const_conv(novel::NOVEL_MAIN, novel::NOVEL_STRUCTURE),
        "Beamer" => const_conv(beamer::BEAMER_MAIN, beamer::BEAMER_STRUCTURE),
        "Lachaise" => const_conv(lachaise::LACHAISE_MAIN, lachaise::LACHAISE_STRUCTURE),
        _ => panic!("Unknown template: {}", template),
    }
}

/// Create Template
/// 
/// Given: 
/// - A project name
/// - A path to the template (Optional)
/// - A template name
/// 
/// # Examples
/// 
/// ```
/// use texcreate_lib::routes::create;
/// 
/// // Recreate the basic template
/// fn main(){
///     let name = "MyProj";
///     let path = "path/";
///     let template = "Basic";
///     create(name, path, template);
/// }
/// ```


pub fn create(name: &str, file_path: &Option<String>, template: &str) {
    // Sanity check for file_path
    let file_path = match file_path{
        Some(path) => path,
        None => "."
    };
    let (main, structure) = load(template);
    let path = format!("{}/{}", file_path, name);
    let def_path = format!("{}/{}_texcreate", file_path, name); //default path 
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