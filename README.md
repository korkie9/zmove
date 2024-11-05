## Zmove allows for seamless file moving in the terminal

### Description
This package builds on [zoxide](https://github.com/ajeetdsouza/zoxide) and ``mv`` shell command (``move`` for Windows) to easily move files in your system.
Move files or directories to any directory you've already been to


### Dependencies
 - [zoxide](https://github.com/ajeetdsouza/zoxide)


NOTE: Support has been added for Zmove on Windows but it is currently untested on Windows

### To install

- Install [zoxide](https://github.com/ajeetdsouza/zoxide)
- Install [cargo](https://github.com/rust-lang/cargo) with ``curl https://sh.rustup.rs -sSf | sh``

- Run ``cargo install zmove``

- Run ``zmv --version`` to verify installation

### Installing manually

- Install [zoxide](https://github.com/ajeetdsouza/zoxide)
- Install [cargo](https://github.com/rust-lang/cargo)
- clone the repository at [zmove](https://github.com/korkie9/zmove)
- Build with ``cargo build --release`` and find executable in release folder


### How to use

- To move a single file or directory: ``zmv <file name> <target directory>``

- To move multiple files or directories: ``zmv *.<extension if file has one> <target directory>``

- To move multiple files or directories: ``zmv <path (optional)>/*.<extension if file has one> <target directory>``

- To move all files or directories: ``zmv <path (optional)>/*.* <target directory>``


Eg. If you have a directory called foobar and you want to move example.txt into it, you could run ``zmv example.txt foobar`` or even just ``zmv example.txt bar`` and full directory path will be inferred giving that the user has visited it before or if foobar exists in current directory.

Eg. If you would like to move multiple files to another directory, run ``zmv *.txt bar`` or ``zmv foo/*.txt bar`` or ``zmv ./* bar``


### Contribute
- Feel free to raise issues and make pull requests at ``https://github.com/korkie9/zmove``
