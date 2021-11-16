pub const MATH_MAIN: &str = r#"% Math Template
% MKProjects | www.mkproj.com
% TexCreate | GPLv2 MIT License 
\documentclass[11pt, letterpaper]{article}
% MetaData goes here
\author{}
\title{}
\date{}
% Packages are in structure.tex
\input{structure.tex}

\begin{document}
    \maketitle
    \pagenumbering{arabic}
    \newpage
    \tableofcontents
    \newpage
    % Code goes here
\end{document}
"#;
pub const MATH_STRUCTURE: &str = r#"% For math equations, theorems, symbols, etc
\usepackage{amsmath}
\usepackage{amsfonts}
\usepackage{amssymb}
\usepackage{amsthm} 

% For pictures
\usepackage{graphicx}
\usepackage{tikz}

% Theorems
\newtheorem{theorem}{Theorem}
\newtheorem{example}{Example}
\newtheorem{remark}{Remark}
\newtheorem{definition}{Definition}
\newtheorem{corollary}{Corollary}
\newtheorem{proposition}{Proposition}

% Extra packages from config.toml goes here
"#;