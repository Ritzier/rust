// s-expr
//
// Parse CommonMark source files and print their AST as S-expressions.
//
// # Usage
//
// $ cargo run --example s-expr file1.md file2.md
// $ cat file.md | cargo run --example s-expr

/// Spaces to indent nested nodes
const INDENT: usize = 4;

/// If true, the close parenthesis is printed in its own line.
const CLOSE_NEWLINE: bool = false;

use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ExtensionOptions, Options};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

fn iter_nodes<'a, W: Write>(
    node: &'a AstNode<'a>,
    writer: &mut W,
    indent: usize,
) -> io::Result<()> {
    use NodeValue::*;

    macro_rules! try_node_inline {
        ($node:expr, $name:ident) => {{
            if let $name(t) = $node {
                return write!(writer, concat!(stringify!($name), "({:?})"), t,);
            }
        }};
    }

    match &node.data.borrow().value {
        Text(t) => write!(writer, "{:?}", t)?,
        value => {
            try_node_inline!(value, FootnoteDefinition);
            try_node_inline!(value, FootnoteReference);
            try_node_inline!(value, HtmlInline);

            if let Code(code) = value {
                return write!(writer, "Code({:?}, {})", code.literal, code.num_backticks);
            }

            let has_blocks = node.children().any(|c| c.data.borrow().value.block());

            write!(writer, "({:?})", value)?;
            for child in node.children() {
                if has_blocks {
                    write!(writer, "\n{1:0$}", indent + INDENT, " ")?;
                } else {
                    write!(writer, " ")?
                }
                iter_nodes(child, writer, indent + INDENT)?
            }

            if indent == 0 {
                write!(writer, "\n)\n")?;
            } else if CLOSE_NEWLINE && has_blocks {
                write!(writer, "\n{1:0$}", indent, " ")?;
            } else {
                write!(writer, ")")?;
            }
        }
    }

    Ok(())
}

fn dump(source: &str) -> io::Result<()> {
    let arena = Arena::new();

    let extension = ExtensionOptions::builder()
        .strikethrough(true)
        .tagfilter(true)
        .table(true)
        .autolink(true)
        .tasklist(true)
        .superscript(true)
        .footnotes(true)
        .description_lists(true)
        .math_dollars(true)
        .math_code(true)
        .wikilinks_title_after_pipe(true)
        .wikilinks_title_before_pipe(true)
        .build();

    let opts = Options {
        extension,
        ..Options::default()
    };

    let doc = parse_document(&arena, source, &opts);

    let mut output = BufWriter::new(io::stdout());
    iter_nodes(doc, &mut output, 0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1).peekable();
    let mut body = String::new();

    if args.peek().is_none() {
        io::stdin().read_to_string(&mut body)?;
        dump(&body)?;
    }

    for filename in args {
        println!("{:?}", filename);
        body.clear();
        File::open(&filename)?.read_to_string(&mut body)?;
        dump(&body)?;
    }

    Ok(())
}
