# Rust Explorer

##
This is a simple project to explore the Rust programming language. It is a file explorer that can open, create, delete, and list files and directories. It can also show and hide hidden and system files on windows, and hidden files on linux.

## Table of Contents
- [Rust Explorer](#rust-explorer)
  - [Table of Contents](#table-of-contents)
  - [Usage](#usage)
  - [Features](#features)
  - [Todo](#todo)
  - [Notes](#notes)
  - [License](#license)

## Usage
To run the project, you need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install). After installing Rust, you can run the project by running the following command in the project directory:
```bash
cargo run
```

## Features
- [x] Traverse directories
- [x] Open files in their default program
- [x] Create files and directories
- [x] Delete files and directories
- [x] Show and hide hidden and system files on windows
- [x] Show hidden files on linux
- [x] Can copy current directory to clipboard
- [x] 

## Todo
- [ ] Fix file and folder size calculation/display
- [ ] Maybe add Caching for faster file listing
- [ ] Fix button sizing and spacing (formatting)
- [ ] File and folder creation dialog open at same time
- [ ] File and folder dialog x button doesn't work
- [ ] Check if a file or folder exists before creating it, otherwise prompt overwrite
- [ ] Cancel button doesn't close delete dialog
- [ ] Make checkbox for showing hidden and system files bigger
- [ ] Add renaming files and folders button

## Notes
- File sizes: (fs::extra::dir::get_size()) (dir size), (std::fs::metadata().len()) (file size)

## License