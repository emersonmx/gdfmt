# `gdfmt` - GDScript Formatter

`gdfmt` is a simple command-line tool for formatting GDScript code. It leverages
Tree-sitter to parse GDScript files into an Abstract Syntax Tree (AST), allowing
for intelligent and consistent code formatting.

## Installation

To install `gdfmt`, you'll need Rust and Cargo installed on your system. If you
don't have them, you can get them from [rustup.rs](https://rustup.rs/).

```bash
cargo install --git https://github.com/emersonmx/gdfmt
```

## Usage

To format a GDScript file, simply run `gdfmt` followed by the path to the file:

```bash
gdfmt path/to/your_file.gd
```

The formatted content will be printed to standard output. 

## Donation

If you find `gdfmt` useful, consider supporting its development:

*   **Ko-fi**: [https://ko-fi.com/emersonmx](https://ko-fi.com/emersonmx)
