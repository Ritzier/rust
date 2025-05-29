use std::fs;

use convert_case::{Case, Casing};
use glob::glob;
use proc_macro2::{Ident, Span};
use quote::quote;

mod description;
mod markdown_parser;
use description::Description;
use markdown_parser::Markdown;

#[proc_macro]
pub fn include_md(_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Target file
    let var_name = concat!("Docs/**/*.md");
    let pattern = var_name;

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

    // Structure for parse markdown
    let markdown = Markdown::new();

    // Process each markdown file
    for (file_name, file_path) in file_list {
        // Convert filename to PascalCase for component names (e.g. "my_blog" -> MyBlog)
        let fn_name = Ident::new(&file_name.to_case(Case::Pascal), Span::call_site());

        // Read markdown content
        let file_str = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(err) => {
                let err = err.to_string();
                return quote!(compile_error!(#err)).into();
            }
        };

        // Parse markdown and extract metadata
        let (header, section) = match markdown.parse_markdown(&file_str, file_name.clone()) {
            Ok(((header, section), description)) => {
                descriptions.push(description);
                (header, section)
            }
            Err(err) => {
                return quote!(compile_error!(#err)).into();
            }
        };

        // Generate component function
        // `my_first_blog` file:
        // #[component] fn My_First_Blog() -> impl IntoView{}
        fn_list.push(quote! {
            #[component]
            fn #fn_name() -> impl IntoView {
                view! {
                    <article>
                        <header>
                            #header
                        </header>
                        <section>
                            #section
                        </section>
                    </article>
                }
                .into_any()
            }
        });

        // Generate route definition
        // <Route path=path!("my_first_blog") view=My_First_Blog />
        route_list.push(quote! {
            <Route path=path!(#file_name) view=#fn_name />
        })
    }

    // Sort blogs by date (newest first)
    Description::reverse_as_date(&mut descriptions);

    // Generate blog listing page component
    fn_list.push(quote! {
        #[component]
        pub fn BlogPage() -> impl IntoView {
            view!{
                <h1>"Blog Page"</h1>
                <div>
                    #(#descriptions)*
                </div>
            }
            .into_any()
        }
    });

    // Generate nested route structure for `BlogRoute`
    fn_list.push(quote! {
        #[component(transparent)]
        pub fn BlogRoute() -> impl MatchNestedRoutes + Clone {
            view! {
                <ParentRoute path=path!("blog") view=|| view!{<Outlet/>}>
                    <Route path=path!("") view=BlogPage />
                    #(#route_list)*
                </ParentRoute>
            }.into_inner()
        }
    });

    // Combine all generated code into a single TokenStream
    quote!(#(#fn_list)*).into()
}
