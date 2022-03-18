 # Let's Begin project
Welcome to project, let's begin with an overview of where all of the files are located.

- `src/`
    - Contains all of the source code
- `out/`
    - Contains the compiled pdf
- `texcreate.toml`
    - Contains the default to compile and view code

## Building and Viewing
To build a project, you can use:
```
$ texcreate compile
# or
$ <texcompiler> --output-directory out/ src/project.tex
```

To view the project, you can use:
```
$ texcreate view
# or
$ <pdfviewer> out/project.pdf
```
