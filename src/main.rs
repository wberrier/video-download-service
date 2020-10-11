
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello")
        .map(|| format!("Hello, World!"));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 8080))
        .await;
}

