use crate::set;
use tex_rs::element::*;
use tex_rs::Latex;

pub fn basic(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str) -> Latex {
    let mut latex = Latex::new();
    set(&mut latex, fs, ps, dc, author, title, date);

    // Write in order
    let mut comments: Vec<Comment> = Vec::new();
    comments.push(Comment::new_comment("Meta data goes here", Level::Meta));
    latex.add_package("amsmath".to_string());
    comments.push(Comment::new_comment(
        "Extra packages from config.toml goes under here",
        Level::Package,
    ));
    let input = UserDefined::new("\\input{structure.tex}", Level::Meta);
    comments.push(Comment::new_comment("Document code goes here", Level::Body));

    let mut elements = Vec::new();
    elements.push(Element::from(comments[0].clone()));
    elements.push(Element::from(comments[1].clone()));
    elements.push(Element::from(input));
    elements.push(Element::from(comments[2].clone()));
    latex.set_elements(&elements);
    latex
}
