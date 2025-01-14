use std::fs;

use chrono::{DateTime, NaiveDateTime, Utc};
use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena,
};
use convert_case::{Case, Casing};
use glob::glob;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

#[proc_macro]
pub fn include_md(_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let pattern = concat!("Docs/**/*.md");

    let file_list: Vec<(String, String)> = glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter_map(|path| {
            let file_name = path.file_stem()?.to_str()?.to_string();
            let file_path = path.to_str()?.to_string();
            Some((file_name, file_path))
        })
        .collect();

    let mut fn_list = vec![];
    let mut route_list = vec![];
    let mut descriptions = vec![];

    for (file_name, file_path) in file_list {
        let fn_name = Ident::new(&file_name.to_case(Case::Pascal), Span::call_site());
        let file_str = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(err) => {
                let err = err.to_string();
                return quote!(compile_error!(#err)).into();
            }
        };

        // Parse str to Token
        let body = match parse_markdown(&file_str, file_name.clone()) {
            Ok((body, description)) => {
                descriptions.push(description);
                body
            }
            Err(err) => {
                return quote!(compile_error!(#err)).into();
            }
        };

        // <#fn_name />
        fn_list.push(quote! {
            #[component]
            pub fn #fn_name() -> impl IntoView {
                view! {
                    #body
                }.into_any()
            }
        });

        // <Route />
        route_list.push(quote! {
            <Route path=path!(#file_name) view=#fn_name />
        })
    }

    // Sort description from latest to oldest
    reverse_descriptions(&mut descriptions);

    // <BlogPage>
    fn_list.push(quote! {
        #[component]
        pub fn BlogPage() -> impl IntoView {
            view! {
                <h1>"Blog Page"</h1>
                <div>
                    #(#descriptions)*
                </div>
            }.into_any()
        }
    });

    // <BlogRoute>
    fn_list.push(quote! {
        #[component(transparent)]
        pub fn BlogRoute() -> impl MatchNestedRoutes + Clone {
            view! {
                <ParentRoute path=path!("blog") view=||view!{<Outlet/>}>
                    <Route path=path!("") view=BlogPage />
                    #(#route_list)*
                </ParentRoute>
            }.into_inner()
        }
    });

    quote!(#(#fn_list)*).into()
}

fn parse_markdown(md_text: &str, path: String) -> Result<(TokenStream, Description), String> {
    let mut description = Description::default();
    description.path = path;

    let arena = Arena::new();
    let mut options = comrak::Options::default();
    options.extension.table = true;

    let root = parse_document(&arena, &md_text, &options);
    let body = iter_nodes(md_text, root, &mut description);

    Ok((body, description))
}

fn iter_nodes<'a>(
    md_text: &str,
    node: &'a AstNode<'a>,
    description: &mut Description,
) -> TokenStream {
    let mut children = vec![];

    for n in node.children() {
        children.push(iter_nodes(md_text, n, description))
    }

    match &node.data.borrow().value {
        NodeValue::Document => quote!(#(#children)*),

        NodeValue::Text(text) => {
            let text = text.clone();
            quote!(#text)
        }

        NodeValue::Paragraph => quote!(
            <p>
                #(#children)*
            </p>
        ),

        NodeValue::Heading(node_heading) => {
            let level = node_heading.level;
            let tag = Ident::new(&format!("h{}", level), Span::call_site());

            // if Description.title empty, add title
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
                quote!(<p>#date_str</p>)
            };

            let tags = if description.tags.is_empty() {
                quote!()
            } else {
                let tags_str = description.tags.clone();
                quote!(<p>#tags_str</p>)
            };

            quote!(
                <#tag>#(#children)*</#tag>
                #date
                #tags
            )
        }

        NodeValue::HtmlBlock(block) => {
            // Check if this is a comment
            if block.literal.starts_with("<!--") && block.literal.ends_with("-->\n") {
                let comment_content = &block.literal[4..block.literal.len() - 4];

                if comment_content.starts_with("Date:") {
                    let date = comment_content[5..].trim();
                    let naive = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S")
                        .unwrap_or_default();
                    description.date = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
                } else if comment_content.starts_with("Tags:") {
                    let tags = comment_content[5..].trim();
                    description.tags = tags.to_string();
                }
            }

            quote!()
        }

        NodeValue::Code(node_code) => {
            let code = node_code.literal.clone();
            quote!(
                <code>
                    #code
                </code>
            )
        }

        node => {
            let node_str = format!("{{{node:?}}} TODO!!!");
            quote!(#node_str)
        }
    }
}

#[derive(Default)]
struct Description {
    title: String,
    date: DateTime<Utc>,
    tags: String,
    path: String,
}

impl ToTokens for Description {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let title = &self.title;
        let path = &self.path;
        let date = &self.date.to_string();
        let tags = &self.tags;
        tokens.extend(quote! {
            <A href=#path>
                <span class="title">#title</span>
                <span class="date">#date</span>
                <span class="tags">#tags</span>
            </A>
        })
    }
}

fn reverse_descriptions(descriptions: &mut Vec<Description>) {
    descriptions.sort_by_key(|description| std::cmp::Reverse(description.date));
}
