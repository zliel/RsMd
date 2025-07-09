# RustMark

RustMark is a 100% Commonmark-compliant Markdown parser and static site generator written in Rust. It is designed to be fast, efficient, and easy to use.

## Table of Contents

<!--toc:start-->

- [RustMark](#rustmark)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation (WIP)](#installation-wip)
  - [Usage](#usage)
    - [Options](#options)
  - [Configuration](#configuration)
  <!--toc:end-->

## Features

- Fast Markdown parsing
- HTML generation
- Custom configuration
- Easy-to-use CLI

## Installation (WIP)

To install RustMark, you need to have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).
Once you have Rust installed, you can install RustMark using Cargo:

```bash
cargo install rustmark
```

## Usage

To use RustMark, you can run the following command in your terminal:

```bash
rust-mark [OPTIONS] <INPUT_DIR>
```

Where `<INPUT_DIR>` is the path to the directory of Markdown files you want to parse. The output will be written to `/output` by default.

### Options

You can also use the following CLI arguments to customize the behavior of RustMark:

- `-c, --config <CONFIG>`: Specify a custom configuration file (default: `./config.toml`).
- `-o, --output-dir <OUTPUT_DIR>`: Specify the output directory for the generated HTML files (default: `/output`).
- `-h, --help`: Display help information.
- `-V, --version`: Display the version of RustMark.

## Configuration

You can customize RustMark's behavior by specifying a config file to use. Here is the default configuration:

```toml
# Tokenization
[lexer]
tab_size = 4

# HTML Generation
[html]
css_file = "default" # "default" for the default styles
favicon_file = ""    # Empty for no favicon
```
