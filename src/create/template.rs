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
\end{document}"#;
pub const MATH_STRUCTURE: &str = r#"% Packages goes here
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{amsfonts}
\usepackage{amsthm}
\usepackage{graphicx}
\usepackage{xcolor}

\newcommand{\intoo}[2]{\mathopen{]}#1\,;#2\mathclose{[}}
\newcommand{\ud}{\mathop{\mathrm{{}d}}\mathopen{}}
\newcommand{\intff}[2]{\mathopen{[}#1\,;#2\mathclose{]}}
\renewcommand{\qedsymbol}{$\blacksquare$}
\newtheorem{notation}{Notation}[chapter]

% Boxed/framed environments
\newtheoremstyle{blacknumbox}% Theorem style name
{0pt}% Space above
{0pt}% Space below
{\normalfont}% Body font
{}% Indent amount
{\small\bf\sffamily\color{black}}% Theorem head font
{\;}% Punctuation after theorem head
{0.25em}% Space after theorem head
{\small\sffamily\color{black}\thmname{#1}\nobreakspace\thmnumber{\@ifnotempty{#1}{}\@upn{#2}}% Theorem text (e.g. Theorem 2.1)
\thmnote{\nobreakspace\the\thm@notefont\sffamily\bfseries\color{black}---\nobreakspace#3.}} % Optional theorem note

\newtheoremstyle{blacknumex}% Theorem style name
{5pt}% Space above
{5pt}% Space below
{\normalfont}% Body font
{} % Indent amount
{\small\bf\sffamily}% Theorem head font
{\;}% Punctuation after theorem head
{0.25em}% Space after theorem head
{\small\sffamily{\tiny\ensuremath{\blacksquare}}\nobreakspace\thmname{#1}\nobreakspace\thmnumber{\@ifnotempty{#1}{}\@upn{#2}}% Theorem text (e.g. Theorem 2.1)
\thmnote{\nobreakspace\the\thm@notefont\sffamily\bfseries---\nobreakspace#3.}}% Optional theorem note

\newtheoremstyle{blacknumbox} % Theorem style name
{0pt}% Space above
{0pt}% Space below
{\normalfont}% Body font
{}% Indent amount
{\small\bf\sffamily}% Theorem head font
{\;}% Punctuation after theorem head
{0.25em}% Space after theorem head
{\small\sffamily\thmname{#1}\nobreakspace\thmnumber{\@ifnotempty{#1}{}\@upn{#2}}% Theorem text (e.g. Theorem 2.1)
\thmnote{\nobreakspace\the\thm@notefont\sffamily\bfseries---\nobreakspace#3.}}% Optional theorem note

% Non-boxed/non-framed environments
\newtheoremstyle{blacknum}% Theorem style name
{5pt}% Space above
{5pt}% Space below
{\normalfont}% Body font
{}% Indent amount
{\small\bf\sffamily\color{black}}% Theorem head font
{\;}% Punctuation after theorem head
{0.25em}% Space after theorem head
{\small\sffamily\color{black}\thmname{#1}\nobreakspace\thmnumber{\@ifnotempty{#1}{}\@upn{#2}}% Theorem text (e.g. Theorem 2.1)
\thmnote{\nobreakspace\the\thm@notefont\sffamily\bfseries\color{black}---\nobreakspace#3.}} % Optional theorem note
\makeatother

% Defines the theorem text style for each type of theorem to one of the three styles above
\newcounter{dummy} 
\numberwithin{dummy}{section}
\theoremstyle{blacknumbox}
\newtheorem{theoremeT}[dummy]{Theorem}
\newtheorem{problem}{Problem}[chapter]
\newtheorem{exerciseT}{Exercise}[chapter]
\theoremstyle{blacknumex}
\newtheorem{exampleT}{Example}[chapter]
\theoremstyle{blacknumbox}
\newtheorem{vocabulary}{Vocabulary}[chapter]
\newtheorem{definitionT}{Definition}[section]
\newtheorem{corollaryT}[dummy]{Corollary}
\theoremstyle{blacknum}
\newtheorem{proposition}[dummy]{Proposition}

% All extra packages from config.toml goes here"#;

pub const THEATRE_MAIN: &str = r#"% Theatre template
% MKProjects 2021 | www.mkproj.com
% TexCreate 
% Used for creating scripts in LaTeX

\documentclass[11pt, letterpaper]{article}
% Packages goes into structure.tex file
\input{structure.tex}
% MetaData goes here
\author{}
\title{}
\date{}

\begin{document}
    \maketitle
    \pagenumbering{arabic}
    \newpage
    \tableofcontents
    \newpage
    % Characters go here 
    \Character[]{}{} %[character description]{character name}{alias}
    % Set length to longest character name 
    \setlength{\speakswidth}{\widthof{\speaksfont Name}}
    \addtolength{\speakswidth}{\Dlabelsep}
    \addtolength{\speaksindent}{\speakswidth}
    % Acts 
    \act[] %[act name]
    \scene[]%[scene name]
    % To make character speak, use \<alias>speaks{text}
\end{document}
"#;
pub const THEATRE_STRUCTURE: &str = r#"% All packages goes here
\usepackage{dramatist}
\usepackage{calc}
\usepackage[utf8]{inputenc}
\usepackage{fancyhdr}
\pagestyle{fancy}
\fancyhf{}
\lhead{\thetitle}
\rhead{\theauthor}
\rfoot{Page \thepage}
% All config.toml extra packages goes here"#;
