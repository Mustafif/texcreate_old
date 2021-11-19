# Introduction
TexCreate is a project built to build LaTex projects without needing to 
annoyingly type out the same commands over and over again. So instead type 
all the metadata in a `config.toml` file and TexCreate will do the rest.

Actually there is two methods to create projects, but the `import` command 
is the recommended one, so we will focus on that.

First let's install the project, and to do that simply run the following: 
```shell
$ cargo install texcreate
```
At the time of writing the current version is 0.7.0, and this guide 
should work from this version and future versions (to check run `texcreate --version`). 

Now let's setup our project, and to start we need a `config.toml`, so why not initialize it?
```shell
$ texcreate init
# Check your directory
$ ls
config.toml
```
When you create a new `config.toml`, you get the following file: 
```toml
[Project]
author = "Author"
title = "Title"
date = "\\today"
project_name = "Project Name"
template = "Math" #Make sure to have first letter upercased

[Document]
paper_size = "letterpaper"
font_size = 11 #font size number
document_class = "article"
packages = ["PhantomData", ""]
```
Wow...so now what do we do, well let's create a project that will give a simple intro on 
the basics of Golang. But...is there a template for that? Well conveniently there's a `list` command 
that gives us a list of the available templates we can use:
```shell
$ texcreate list
//////////////////////////////////////
// List of available templates:
//////////////////////////////////////
// Basic => A simple project with the basic setup you need
// Book => A project that utilizes both a markdown and tex format
// Math => A project that utilizes math related packages
// Theatre => A project that is ideal to write scripts
// Code => A project ideal to document about programming
// Novel => A project to begin writing that novel you always delay
// Beamer => A project to start making presentations in LaTeX
```
Let's use the `Code` template and we will continue in the next section.