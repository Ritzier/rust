# Markdown to Leptos

Using a proc-macro to generate Leptos components from Markdown files at compile time enables you to write content in
Markdown and automatically expose it as interactive Leptos views.

## `build.rs`

In Cargo, a `proc-macro` crate is compiled just once, and by default, it won’t recompile unless its own source code or
dependencies change.

`markdown/build.rs`:

```rs
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");

    println!("cargo:rerun-if-changed=../Docs/");

    // Watch every file in the folder recursively
    for entry in WalkDir::new("../Docs").into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
}
```

## How it work

1. \*Write Markdown files\*\* in the `Docs/` folder
2. _During compilation_, the `proc-macro` (`markdown` member):
   - Scans the `Docs/` folder for `.md` files
   - Parses each Markdown file
   - Generates a Leptos component function for each file.
   - Generates route definitions to match the file names
