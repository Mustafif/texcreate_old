pub const NOVEL_MAIN: &str = r#"% Novel Template
% MKProjects 2021 | TexCreate
% MIT & GPLv2 License
\documentclass[{font_size}pt, {papaer_size}, oneside]{{doc_class}}

\input{structure.tex}

\author{{author}}
\title{{title}}
\date{{date}}

\begin{document}
    \maketitle
    \newpage
    \tableofcontents
    \pagenumbering{arabic}
    \newpage
    \chapter{Some Chapter}\label{ch:some-chapter}
    \lipsum*[20]
    \newpage
    \lipsum*[20]
\end{document}
"#;

pub const NOVEL_STRUCTURE: &str = r#"\usepackage{fancyhdr}
\usepackage{xcolor}
\usepackage{lipsum}
\pagestyle{fancy}
\fancyhf{}
\lhead{\leftmark}
\rhead{\rightmark}
\rfoot{Page \thepage}

\setlength{\headheight}{14.5pt}
% Extra packages from config.toml here
"#;