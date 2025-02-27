use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};

fn run_with(builder: SyntectAdapterBuilder) {
    let adapter = builder.build();
    let options = Options::default();
    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let input = concat!("```Rust\n", "fn main();\n", "```");

    let formatted = markdown_to_html_with_plugins(input, &options, &plugins);

    println!("{}", formatted);
}

fn main() {
    run_with(SyntectAdapterBuilder::new().theme("base16-ocean.dark"));
    run_with(SyntectAdapterBuilder::new().css())
}
