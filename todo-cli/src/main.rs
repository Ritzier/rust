mod app;

use app::App;
use inquire::error::InquireResult;

fn main() -> InquireResult<()> {
    App::new("todo.json".into()).run()
}
