use tex_rs::{Latex, UserDefined};
use crate::set;

pub fn beamer(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str) -> Latex{
    let mut latex = Latex::new();
    set(&mut latex, fs, ps, dc, author, title, date);

    let input = UserDefined::new()

    latex
}