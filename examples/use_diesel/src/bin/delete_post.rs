use diesel::prelude::*;
use std::env::args;
use use_diesel::*;

fn main() {
    use schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target a match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
