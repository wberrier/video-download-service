use anyhow::Result;
use std::collections::HashMap;
use warp::Filter;
use std::process::Command;

#[macro_use]
extern crate anyhow;

use video_download_service::templates::*;

async fn display_index() -> Result<impl warp::Reply, warp::Rejection> {
    let doc_res = TEMPLATE_ENGINE.render("index.html", &{});

    match doc_res {
        Ok(document) => Ok(warp::reply::html(document)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn display_download(
    query_map: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match query_map.get("url") {
        Some(url) => {
            match handle_download(url) {
                Ok(_) => {
                    Ok(warp::reply::html("Download successful"))
                },
                Err(_) => {
                    // TODO: how to satisfy the borrow checker???
                    //     error_document = TEMPLATE_ENGINE.render("error.html", &{}).unwrap();
                    //     Ok(warp::reply::html(error_document.as_str()))
                    Ok(warp::reply::html("Error Downloading Video"))
                }
            }
        }
        None => Err(warp::reject::not_found()),
    }
}

fn handle_download(url: &str) -> Result<()> {
    println!("Downloading url: '{}'", url);

    match Command::new("sh")
        .arg("-c")
        .arg(format!("youtube-dl {}", url))
        .output() {
            Ok(output) => {
                if output.status.success() {
                    println!("Output: {}", std::str::from_utf8(&output.stdout).unwrap());
                    Ok(())
                } else {
                    Err(anyhow!("youtube-dl failed"))
                }
            },
            Err(_) => {
                Err(anyhow!("Error executing youtube-dl"))
            }
        }
}

#[tokio::main]
async fn main() {
    // GET / => 200 OK with index body
    let main_page = warp::get().and(warp::path::end()).and_then(display_index);

    let download_page = warp::get()
        .and(warp::path("download"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(display_download);

    let routes = main_page.or(download_page);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
