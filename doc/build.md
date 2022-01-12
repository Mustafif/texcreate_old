# Build
The build command is used to build a LaTeX project using a `config.toml` file.
This command contains two optional arguments:  
1. `--file`: The path to the `config.toml` file
2. `--mode`: To specify the build mode, the default is `single`
    - `single`: Builds a single project 
    - `multi` : Builds multiple projects

Let's try out the build command:
```shell
# Initialize a multi project 
$ texcreate init --mode multi
$ ls 
config.toml 
```
Let's build the project:
```shell
$ texcreate build --mode multi
$ ls
config.toml Project1 Project2
```
