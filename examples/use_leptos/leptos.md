# How does Leptos Work?

Leptos is a modern Rust framework designed for building full-stack web applications. It supports **Server-Side Rendering
(SSR)**, **Client-Side Rendering (CSR)**, and **hydration**, enabling seamless isomorphic (shared server-client)
applications.

1. Server-Side Rendering (SSR)

   - Leptos can render HTML on the server and send it to the client.
   - The improves initial page load times and SEO because the browser receives fully-rendered HTML.
   - Example:

   ```rust
   #[actix_web::main]
   async fn main() -> std::io::Result<()> {
   HttpServer::new(move || {
       let routes = generate_route_list(Counters); // Generates routes for the app
       App::new()
           .leptos_routes(routes, || {
               view! {
                   <!DOCTYPE html>
                   <html>
                       <head>
                           <meta charset="utf-8" />
                           <title>"Leptos App"</title>
                       </head>
                       <body>
                           <Counters />
                       </body>
                   </html>
               }
           })
   })
   .bind("127.0.0.1:3000")?
   .run()
   .await
   }
   ```

2. Client-Side Hydration

   - After the server sends pre-rendered HTML, Leptos hydrates it on the client.
   - Hydration attaches event listeners and restore interactivity to the static HTML
   - Example:

   ```rust
   #[cfg(feature = "hydrate")]
    #[wasm_bindgen::prelude::wasm_bindgen]
    pub fn hydrate() {
        leptos::mount::hydrate_body(Counters);
    }
   ```

   - This function activates interactivity in components like buttons or forms.

3. Isomorphic Server Functions

   - Leptos allows you to define functions that can be called from both the server and client using the `#[server]`
     attribute.
   - Example:

   ```rust
   #[server]
   pub async fn get_server_count() -> Result<i32, ServerFnError> {
        Ok(42) // Example of fetching data from the server
   }
   ```

   - These functions run on the server but can be triggered from client-side code.

4. Read-Time Updates

   - Leptos supports real-time updates using technologies like Server-Send Events (SSE)
   - Example:

   ```rust
   #[get("/api/events")]
   async fn counter_events() -> impl Responder {
       HttpResponse::Ok()
           .insert_header(("Content-Type", "text-/event-stream"))
           .streaming(some_stream)
   }
   ```

   - This allows features like live counters or collaborative editing.

## What is Hydration?

**Hydration** is the process of attaching interactivity to a pre-rendered HTML page. In Leptos:

1. The server renders static HTML using SSR.

2. The client-side WebAssembly (WASM) reactivates this HTML by attaching event listeners and restoring state

## Why is Hydration Important?

- **Improved Performance**: Users see content immediately because of SSR, while hydration adds interactivity after
  loading.
- **SEO-Friendly**: Search engines can crawl fully-rendered HTML.
- **Isomorphic Behavior**: Shared code between server and client ensures consistency
