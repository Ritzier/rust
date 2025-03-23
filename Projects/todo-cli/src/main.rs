mod app;

use app::App;
use app::Result;

fn main() -> Result<()> {
    App::new("todo.json".into())?.run()
}
