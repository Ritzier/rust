# Diesel

**Diesel** is an ORM framework (like middleware between Database and Rust),
used to simplify database operations in Rust

[Official Website](https://diesel.rs/)
[Github](https://github.com/diesel-rs/diesel)

## Project Startup

1. Startup Postgres

```sh
podman compose start
```

2. Setup Diesel

```sh
source .env

diesel setup
```

This will create our database (if it didn't exist) and set up the initial
migrations directory, which will contain a generated migration file that
established the Diesel setup. Note that the migrations directory will not be
empty as the initial setup migration is automically generated.

3. Generate table

```sh
diesel migration generate create_posts
```

Diesel CLI will create two empty files for us in the required structure. You'll
see output that looks something like this:

```sh
Creating migrations/2024-11-21-082037_create_posts/up.sql
Creating migrations/2024-11-21-082037_create_posts/down.sql
```

4. Write database schema

`migrations/2024-11-21-082037_create_posts/up.sql`

```sql
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title CARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)
```

`migrations/2024-11-21-082037_create_posts/down.sql`

```sql
DROP TABLE posts
```

5. Apply migration

Apply new migration

```sh
diesel migration run
```

It’s a good idea to ensure that **down.sql** is correct. You can quickly confirm
that your **down.sql** rolls back your migration correctly by redoing the migration:

```sh
diesel migration redo
```

Diesel would auto generate `src/schema.rs` file:

```rust
// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
```

6.
