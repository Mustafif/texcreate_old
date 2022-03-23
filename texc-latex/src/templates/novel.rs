use crate::set;
use tex_rs::*;

type E = Element;
const NOVEL_STRUCT: &str = r#"\pagestyle{fancy}
\fancyhf{}
\lhead{\leftmark}
\rhead{\rightmark}
\rfoot{Page \thepage}
\setlength{\headheight}{14.5pt}
"#;

pub fn novel(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str) -> Latex {
    let mut latex = Latex::new();
    if dc != "book" {
        let dc = "book";
    }
    set(&mut latex, fs, ps, dc, author, title, date);

    let input = UserDefined::new("\\input{src/structure.tex}", Level::Meta);
    let toc = UserDefined::new(r"\tableofcontents", Level::Body);
    let newpage = UserDefined::new(r"\newpage", Level::Body);

    let label_some_chapter = UserDefined::new(r"\label{ch:some-chapter}", Level::Body);
    let lipsum = UserDefined::new(r"\lipsum*[20]", Level::Body);
    let mut some_chapter = Chapter::new("Some Chapter");
    some_chapter.attach(E::from(label_some_chapter));
    some_chapter.attach(E::from(lipsum.clone()));

    let novel_struct = UserDefined::new(NOVEL_STRUCT, Level::Package);

    latex.add_package("xcolor".to_string());
    latex.add_package("lipsum".to_string());

    latex.set_elements(&vec![
        E::from(novel_struct),
        E::from(input),
        E::from(toc),
        E::from(newpage.clone()),
        E::from(some_chapter),
        E::from(newpage.clone()),
        E::from(lipsum.clone()),
    ]);
    latex
}
