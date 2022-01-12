# Create
The create command creates a new project without creating a `config.toml`, so 
to use this command we specify the following arguments:

- `-t/--template`: The template to use
- `-n/--name`: The name of the project
- `-d/--directory`: The directory to create the project in (_Optional_)

```shell
$ texcreate create -t Math -n MyProject -d /path/to/project
$ texcreate create -t Code -n AnotherProject 
# AnotherProject will be created in the current directory
```