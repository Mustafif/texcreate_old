use tex_rs::*;
use crate::set;


/// Creates the beamer template in `tex_rs::Latex`
/// ```
/// use texc_latex::beamer;
// ///
// /// fn main(){
// ///     let beamer_latex = beamer::beamer(11, "letterpaper", "article", "author", "title", "Some day", &vec![]);
// ///     // You can write with the following:
// ///     // beamer_latex.write(...)
// ///     // beamer_latex.async_write(...)
// ///     // beamer_latex.split_write(...), used in texcreate
// /// }
/// ```
pub fn beamer(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str, packages: &Vec<String>) -> Latex{
    let mut latex = Latex::new();
    set(&mut latex, fs, ps, dc, author, title, date);

    let struct_input = UserDefined::new(r"\input{src/structure.tex}", Level::Meta);
    let institute = UserDefined::new(r"\institute{}", Level::Meta);


    let frames = UserDefined::new(r#"% Title Page of the presentation
    \frame{\titlepage}
% New frame for table of contents
    \begin{frame}
        \frametitle{Table of Contents}
        \tableofcontents
    \end{frame}
% New frame for random information
    \begin{frame}
        \section{Introduction}
        \frametitle{Random Information}
        \begin{itemize}
            \item This is a random item
            \item This is another random item
        \end{itemize}]
        \begin{block}{Remarks}
            This is a random remark
        \end{block}
        \begin{alertblock}{Important Alert}
            This is an important random alert
        \end{alertblock}
    \end{frame}
    \section{Another Section}
    \begin{frame}
        \frametitle{Current Table of Contents}
        \tableofcontents[currentsection]
    \end{frame}
    "#, Level::Body);

    let theme = UserDefined::new(r"\usetheme{Warsaw}", Level::Package);

    latex.add_package("amsmath".to_string());
    latex.add_package("listings".to_string());
    latex.add_package("xcolor".to_string());
    latex.add_package("xcolor".to_string());
    latex.add_package("graphicx".to_string());

    for i in packages{
        latex.add_package(i.to_string());
    }

    latex.set_elements(elements![struct_input, theme, institute, frames]);


    latex
}