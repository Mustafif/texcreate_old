pub const CODE_MAIN: &str = r#"% Code Template 
% MKProjects TexCreate 2021 
% MIT & GPLv2 License
\documentclass[11pt, letterpaper]{article}
\input{structure.tex}
\author{}
\date{}
\title{}

\begin{document}
\maketitle
\newpage
\tableofcontents
\newpage
\section{Intro}
% To use listings with code on text, use the following command:
\begin{lstlisting}
#include <stdio.h>
int main()
{
    printf("Hello World!");
    return 0;
}
/*Comments*/
\end{lstlisting}
% To use external code, use the following command:
%\lstinputlisting{file.c}
\end{document}
"#;

pub const CODE_STRUCTURE: &str = r#"\usepackage{graphicx}
\usepackage{listings}
\usepackage{xcolor}
\usepackage{amsmath}
\definecolor{codegreen}{rgb}{0,0.6,0}
\definecolor{codegray}{rgb}{0.5,0.5,0.5}
\definecolor{codepurple}{rgb}{0.58,0,0.82}
\definecolor{backcolour}{rgb}{0.95,0.95,0.92}
\lstdefinestyle{lang_style}{
    backgroundcolor=\color{backcolour},
    commentstyle=\color{codegreen},
    keywordstyle=\color{magenta},
    numberstyle=\tiny\color{codegray},
    stringstyle=\color{codepurple},
    basicstyle=\ttfamily\footnotesize,
    breakatwhitespace=false,
    breaklines=true,
    captionpos=b,
    keepspaces=true,
    numbers=left,
    numbersep=5pt,
    showspaces=false,
    showstringspaces=false,
    showtabs=false,
    tabsize=2
}
\lstset{language=c}
\lstset{style=lang_style}

% Extra packages from config.toml here
"#;