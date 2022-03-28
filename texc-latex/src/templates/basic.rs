use crate::set;
use tex_rs::Comment;
use tex_rs::Latex;
use tex_rs::*;

pub fn basic(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str, packages: &Vec<String>) -> Latex {
    let mut latex = Latex::new();
    set(&mut latex, fs, ps, dc, author, title, date);

    // Write in order
    let mut comments: Vec<Comment> = Vec::new();
    comments.push(Comment::new_comment("Meta data goes here", Level::Meta));
    latex.add_package("amsmath".to_string());
    for i in packages{
        latex.add_package(i.to_string());
    }
    comments.push(Comment::new_comment(
        "Extra packages from config.toml goes under here",
        Level::Package,
    ));
    let input = UserDefined::new("\\input{src/structure.tex}", Level::Meta);
    comments.push(Comment::new_comment("Document code goes here", Level::Body));

    latex.set_elements(elements![
        comments[0].clone(),
        comments[1].clone(),
        input,
        comments[2].clone()
    ]);
    latex
}
