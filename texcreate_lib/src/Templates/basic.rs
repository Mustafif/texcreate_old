pub const BASIC_MAIN: &str = r#"% Basic Template
% MKProjects | GPLv2 & MIT License 
% From TexCreate
\documentclass[11pt, letterpaper]{article}
% MetaData Here
\author{}
\title{}
\date{}
%Packages goes in structure.tex 
\input{structure.tex}
\begin{document}
    \maketitle
    \pagenumbering{arabic}
    \newpage
    % Document code here
\end{document}
"#;

pub const BASIC_STRUCTURE: &str = r#"\usepackage{amsmath}
% Extra packages from config.toml goes under here
"#;