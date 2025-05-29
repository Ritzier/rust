use chrono::{DateTime, NaiveDateTime, Utc};
use comrak::nodes::{AstNode, NodeValue};
use comrak::{Arena, Options, parse_document};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::description::Description;

pub struct Markdown<'a> {
    options: Options<'a>,
}

impl Markdown<'_> {
    pub fn new() -> Self {
        let mut options = Options::default();
        options.extension.table = true;

        Self { options }
    }

    /// Parses markdown content and extracts both rendered body and metadata
    pub fn parse_markdown(
        &self,
        md_text: &str,
        path: String,
    ) -> Result<((TokenStream, TokenStream), Description), String> {
        let mut description = Description::new(path);

        let arena = Arena::new();
        let root = parse_document(&arena, md_text, &self.options);
        let body = Self::parse_nodes(root, &mut description);

        Ok((body, description))
    }

    /// Recursively processes markdown AST nodes to generate view components
    fn parse_nodes<'a>(
        node: &'a AstNode<'a>,
        description: &mut Description,
    ) -> (TokenStream, TokenStream) {
        let mut header_children = vec![];
        let mut section_children = vec![];

        // Process child nodes
        for n in node.children() {
            let (header, section) = Self::parse_nodes(n, description);
            header_children.push(header);
            section_children.push(section);
        }

        match &node.data.borrow().value {
            // Document root - combine all child nodes
            NodeValue::Document => (quote!(#(#header_children)*), quote!(#(#section_children)*)),

            // Text  node - render directly
            NodeValue::Text(text) => {
                let text = text.clone();
                (quote!(), quote!(#text))
            }

            // HTML comment metadata (e.g. <!-- Date: ... -->)
            NodeValue::HtmlBlock(block) => {
                if block.literal.starts_with("<!--") && block.literal.ends_with("-->\n") {
                    let comment_content = &block.literal[4..block.literal.len() - 4].trim();

                    // Extract date from comment
                    if let Some(date) = comment_content.strip_prefix("Date:") {
                        let naive = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
                            .unwrap_or_default();
                        description.date = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
                    }
                    // Extract tags from comment
                    if let Some(tags) = comment_content.strip_prefix("Tags:") {
                        description.tags = tags.to_string();
                    }
                }

                (quote!(), quote!())
            }

            // Heading with metadata extraction
            NodeValue::Heading(node_heading) => {
                let level = node_heading.level;
                let tag = Ident::new(&format!("h{level}"), Span::call_site());

                match level {
                    // Handle H1 as header with metadata
                    1 => {
                        if description.title.is_empty() {
                            let title_text: String = node
                                .children()
                                .filter_map(|child| {
                                    if let NodeValue::Text(text) = &child.data.borrow().value {
                                        Some(text.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            description.title = title_text;
                        }

                        let date = {
                            let date_str = description.date.to_string();
                            quote!(<span class="date">#date_str</span>)
                        };

                        let tags = if !description.tags.is_empty() {
                            let tags_str = description.tags.clone();
                            quote!(<span class="tags">#tags_str</span>)
                        } else {
                            quote!()
                        };

                        (
                            quote! {
                                <#tag>#(#section_children)*</#tag>
                                <p class="meta">
                                    #date
                                    #tags
                                </p>
                            },
                            quote!(),
                        )
                    }
                    _ => (
                        quote!(),
                        quote! {
                                <#tag>#(#section_children)*</#tag>
                        },
                    ),
                }
            }

            // Paragraph wrapping — belongs to section content
            NodeValue::Paragraph => {
                let content = quote! {
                    <p>
                        #(#section_children)*
                    </p>
                };

                (quote!(), content)
            }

            // Warning node while compiling
            node => {
                eprintln!("Warning: Unhandled node encountered: {:?}", node);
                (quote!(), quote!())
            }
        }
    }
}
