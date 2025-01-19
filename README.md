# mim
A command line program for running bash commands with Anthropic Haiku model.  

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
