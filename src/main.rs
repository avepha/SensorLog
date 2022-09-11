pub mod api;
pub mod db;
pub mod handlers;
pub mod models;
pub mod server;
pub mod tests;
pub mod utils;

extern crate lazy_static;

#[tokio::main]
async fn main() {
    println!("Logger service is ready at http://localhost:3030");
    warp::serve(server::start())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
