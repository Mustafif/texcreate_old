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

pub const THEATRE_MAIN: &str = r#"% Theatre template
% MKProjects 2021 | www.mkproj.com
% TexCreate 
% Used for creating scripts in LaTeX

\documentclass[11pt, letterpaper]{article}
% Packages goes into structure.tex file
\input{structure.tex}
% MetaData goes here
\author{Author}
\title{Title}
\date{\today}

\begin{document}
    \maketitle
    \pagenumbering{arabic}
    \newpage
    \tableofcontents
    % Characters go here
    \Character[bob]{bob}{bob} %[character description]{character name}{alias}
    % Set length to longest character name
    \setlength{\speakswidth}{\widthof{\speaksfont Name}}
    \addtolength{\speakswidth}{\Dlabelsep}
    \addtolength{\speaksindent}{\speakswidth}
    % Acts
    \act[]
    \scene[]
    % To make character speak, use \<alias>speaks{text}
    \begin{drama}
    \bobspeaks{Bob is speaking}
    \end{drama}
\end{document
"#;
pub const THEATRE_STRUCTURE: &str = r#"% All packages goes here
\usepackage{dramatist}
\usepackage{calc}
\usepackage[utf8]{inputenc}
\usepackage{fancyhdr}
% All config.toml extra packages goes here\usepackage{}
"#;
