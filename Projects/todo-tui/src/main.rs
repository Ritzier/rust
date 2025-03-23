use app::App;
mod app;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let file = "todo.json";
    let app = App::new(file.into())?.run(terminal);
    ratatui::restore();
    app
}
