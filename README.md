# mim
A command line program for running bash commands with Anthropic Claude model.  

To install 
```
git clone https://github.com/foomprep/mim.git
cd mim
cargo install --path .
```
Running
```
mim list all files
```
will display the bash command to fulfill request and y/n prompt.

To bypass y/n and execute automatically call with flag
```
mim -y list all files
```

You can pipe content to mim to provide context:
```
cat file.txt | mim format this data as json
```

You can pipe input and still get an execution prompt (on Unix-based systems):
```
cat file.txt | mim format this data as json
```

In non-interactive environments or scripts, use the -y flag to execute commands automatically:
```
cat file.txt | mim -y format this data as json
```

To ignore piped input even when available:
```
cat file.txt | mim -n create a new text file
```

To display the version information:
```
mim --version
```
