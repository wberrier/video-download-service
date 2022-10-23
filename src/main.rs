use anyhow::Result;
use std::collections::HashMap;
use std::fs::{DirEntry, read_dir};
use std::process::Command;
use warp::Filter;
use urlencoding::encode;

#[macro_use]
extern crate anyhow;

use video_download_service::config::*;
use video_download_service::templates::*;

static DOWNLOAD_COMMAND: &'static str = "yt-dlp";

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
            let value_map: HashMap<&str, String> = [("url", url.clone())].iter().cloned().collect();

            match handle_download(url) {
                Ok(_) => {
                    let document = TEMPLATE_ENGINE.render("finished.html", &value_map).unwrap();
                    Ok(warp::reply::html(document))
                }
                Err(error) => {
                    println!("Error: {}", error);
                    let error_value_map: HashMap<&str, String> =
                        [("error", format!("Error downloading url: {}: {}", url, error))]
                            .iter()
                            .cloned()
                            .collect();

                    let document = TEMPLATE_ENGINE
                        .render("error.html", &error_value_map)
                        .unwrap();

                    Ok(warp::reply::html(document))
                }
            }
        }
        None => Err(warp::reject::not_found()),
    }
}

fn handle_download(url: &str) -> Result<()> {
    // Get download directory
    // how to make sure we can change dir without affecting other clients?
    let download_dir = &CONFIG.download_dir;

    println!("Downloading url: '{}' to: '{}'", url, download_dir);

    let command = format!("cd {}; {} --no-mtime '{}'", download_dir, DOWNLOAD_COMMAND, url);

    println!("With command: {}", command);

    // no-mtime: Use the download time for the timestamp so the listing order is based on download time
    match Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => {
            let output_string = std::str::from_utf8(&output.stdout).unwrap();
            let error_string = std::str::from_utf8(&output.stderr).unwrap();
            if output.status.success() {
                println!("Output: {}", output_string);
                println!("Error output: {}", error_string);
                Ok(())
            } else {
                Err(anyhow!("yt-dlp failed: {}: {}", output_string, error_string))
            }
        }
        Err(_) => Err(anyhow!("Error executing yt-dlp")),
    }
}

fn dir_listing(directory: &str) -> Result<std::vec::Vec<DirEntry>> {
    let mut results: std::vec::Vec<DirEntry> = std::vec::Vec::new();

    // TODO: need to sort these by date
    // vector insert with compare func?

    for entry in read_dir(directory)? {
        if entry.is_ok() {
            results.push(entry?);
        }
    }

    // Sort by date so the listing reflects download time
    results.sort_by(|a, b| {
        a.metadata().unwrap().modified().unwrap().cmp(
            &b.metadata().unwrap().modified().unwrap())
    });

    results.reverse();

    Ok(results)
}

async fn list_downloads() -> Result<impl warp::Reply, warp::Rejection> {
    let mut html_list: String = String::new();

    html_list.push_str("<ul>\n");

    // TODO: huh?  How to iterate a Result<Vec> ?
    for list in dir_listing(&CONFIG.download_dir) {
        for entry in list {

            let file = entry.file_name().to_str().unwrap().to_string();

            // Url encode the filename
            let encoded_file = encode(&file);

            html_list
                .push_str(format!("<li><a href=\"file/{}\">{}</a></li>\n", encoded_file, file).as_str());
        }
    }

    html_list.push_str("</ul>\n");

    let value_map: HashMap<&str, String> = [("filelist", html_list)].iter().cloned().collect();

    let document = TEMPLATE_ENGINE.render("filelist.html", &value_map).unwrap();

    Ok(warp::reply::html(document))
}

#[tokio::main]
async fn main() {
    // GET / => 200 OK with index body
    let main_page = warp::get().and(warp::path::end()).and_then(display_index);

    let download_page = warp::get()
        .and(warp::path("download"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(display_download);

    println!("Download dir: {}", &CONFIG.download_dir);

    let file_browser = warp::path("file").and(warp::fs::dir(&CONFIG.download_dir));

    let file_listing = warp::get()
        .and(warp::path("filelist"))
        .and(warp::path::end())
        .and_then(list_downloads);

    let routes = main_page
        .or(download_page)
        .or(file_browser)
        .or(file_listing);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
