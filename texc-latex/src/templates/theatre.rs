use crate::set;
use tex_rs::*;

/// Creates the theatre template in `tex_rs::Latex`
/// ```
/// use texc_latex::theatre;
// ///
// /// fn main(){
// ///     let theatre_latex = theatre::theatre(11, "letterpaper", "article", "author", "title", "Some day", &vec![]);
// ///     // You can write with the following:
// ///     // theatre_latex.write(...)
// ///     // theatre_latex.async_write(...)
// ///     // theatre_latex.split_write(...), used in texcreate
// /// }
/// ```

pub fn theatre(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str, packages: &Vec<String>) -> Latex {
    let mut latex = Latex::new();
    set(&mut latex, fs, ps, dc, author, title, date);

    let struct_input = UserDefined::new(r"\input{src/structure.tex}", Level::Meta);

    let toc = UserDefined::new(r"\tableofcontents", Level::Body);
    let newpage = UserDefined::new(r"\newpage", Level::Body);

    let character = UserDefined::new(r"\Character[Description]{Name}{aliasName}", Level::Body);

    let set_length = UserDefined::new(
        r"\setlength{\speakswidth}{\widthof{\speaksfont Long Name}}",
        Level::Body,
    );
    let add_to_length1 = UserDefined::new(r"\addtolength{\speakswidth}{\Dlabelsep}", Level::Body);
    let add_to_length2 =
        UserDefined::new(r"\addtolength{\speaksindent}{\speakswidth}", Level::Body);

    let act = UserDefined::new(r"\act[] %[act name]", Level::Body);
    let scene = UserDefined::new(r"\scene[]%[scene name]", Level::Body);
    let end_comment = UserDefined::new(
        r" % To make character speak, use \<alias>speaks{text}",
        Level::Body,
    );

    latex.add_package("dramatist".to_string());
    latex.add_package("calc".to_string());
    latex.add_package("fancyhdr".to_string());

    for i in packages{
        latex.add_package(i.to_string());
    }
    let extra_struct = UserDefined::new(
        r#"\usepackage[utf8]{inputenc}
\pagestyle{fancy}
\fancyhf{}
\lhead{\thetitle}
\rhead{\theauthor}
\rfoot{Page \thepage}"#,
        Level::Package,
    );

    latex.set_elements(elements![
        struct_input,
        extra_struct,
        toc,
        newpage,
        character,
        set_length,
        add_to_length1,
        add_to_length2,
        act,
        scene,
        end_comment
    ]);

    latex
}
