## Comrak

[**Comrak**](https://docs.rs/comrak/latest/comrak/) is a Rust library for parsing and rendering Markdown, compliant with
the [**CommonMark**](https://commonmark.org/) specification and [**GFM**](https://github.github.com/gfm/)(GitHub
Flavored Markdown). It is a 1:1 Rust port of GitHub'. `cmark-gfm`, ensuring compatibility with upstream changes. Key
faetures include:

- **Markdown to HTML conversion**: Use `comrak::markdown_to_html` for simple conversions
- **AST manipulation**: Parse Markdown into an Abstract Syntax Tree (AST) for custom transformations
- **GFM extensions**: Supports tables, strikethrough, autolinks and more
- **Safe-by-default**: Scrubs raw HTML and dangerous links unless explicitly allowed
- **Performance**: While slightly slower than some alternatives (e.g., pulldown-cmark), it remains efficient and
  production-ready, used in tools like `docs.rs`

```rust
use comrak::{markdown_to_html, ComrakOPtions};
let html = markdown_to_html("Hello, **world**!", &ComrakOptions::default());
assert_eq!(html, "<p>Hello, <strong>world</strong>!</p>\n")
```

## Quote

[**Quote**](https://docs.rs/quote/latest/quote/) is primarily used for generating Rust code within procedural macros. It
provides a convenient way to create Rust syntax trees and token streams. The main features are:

- **`quote!`** macro for quasi-quoting Rust code
- Abilitiy to interpolate runtime values into the generated code
- Intergration with the `Syn` crate for parsing and manipulating Rust syntax

## Proc-Macro2

<!--TODO: proc-macro2 documentation -->

[**Proc-Macro2**](https://docs.rs/proc-macro2/latest/proc_macro2/)

## Syn

<!--TODO: syn documentation -->

[**Syn**](https://docs.rs/syn/latest/syn/)

## Syntect

<!--TODO: syntect documentation-->

[**Syntect**](https://docs.rs/syntect/latest/syntect/)
