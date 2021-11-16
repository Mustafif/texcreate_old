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