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

/// Creates the novel template in `tex_rs::Latex`
/// ```
/// use texc_latex::novel;
// ///
// /// fn main(){
// ///     let novel_latex = novel::novel(11, "letterpaper", "article", "author", "title", "Some day", &vec![]);
// ///     // You can write with the following:
// ///     // novel_latex.write(...)
// ///     // novel_latex.async_write(...)
// ///     // novel_latex.split_write(...), used in texcreate
// /// }
/// ```

pub fn novel(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str, packages: &Vec<String>) -> Latex {
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

    for i in packages{
        latex.add_package(i.to_string());
    }
    latex.set_elements(elements![
        novel_struct,
        input,
        toc,
        newpage.clone(),
        some_chapter,
        newpage.clone(),
        lipsum.clone()
    ]);
    latex
}
