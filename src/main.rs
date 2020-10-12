
use warp::Filter;

async fn display_index() -> Result<impl warp::Reply, warp::Rejection> {

        Ok(warp::reply::with_status(
            "Welcome to the downloader service",
            warp::http::StatusCode::CREATED,
        ))
}

#[tokio::main]
async fn main() {

    // GET / => 200 OK with index body
    let main_page = warp::get()
        .and(warp::path::end())
        .and_then(display_index);

    warp::serve(main_page)
        .run(([0, 0, 0, 0], 8080))
        .await;
}

