pub const BEAMER_MAIN : &str = r#"% Beamer Template 
% MKProjects | TexCreate
% Mit & GPlv2 License
\documentclass{{doc_class}}
// No use for {paper_size} and {font_size}pt in Beamer
\title{{title}}
\author{{author}}
\date{{date}}
\institute{}
\input{structure.tex}
\begin{document}
% Title Page of the presentation
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
\end{document}
"#;
pub const BEAMER_STRUCTURE : &str = r#"\usetheme{Warsaw}
% Visit https://www.overleaf.com/learn/latex/Beamer#Introduction for more information on beamer.
\usepackage{amsmath}
\usepackage{listings}
\usepackage{xcolor}
\usepackage{graphicx}
% Extra packages from config.toml here
"#;

