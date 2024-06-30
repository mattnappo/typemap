# Typemap
Visualize dependencies between user-defined types in your Rust projects.

## Usage

```
Visualize type dependence in your Rust projects

Usage: typemap [OPTIONS] --infile <INFILE>

Options:
  -i, --infile <INFILE>    Rust file to analyze
  -o, --outfile <OUTFILE>  PDF file to output to. If none, will print dot to stdout
  -h, --help               Print help
  -V, --version            Print version
```

## Limitations
⚠️ This project is not complete ⚠️

Right now Typemap only supports single-file Rust projects with flat module structure.

