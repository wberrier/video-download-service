use anyhow::{anyhow, Result};
use axum::response::Html;
use axum::{extract::Query, routing::get, Router};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::fs::read_dir;
use tokio::process::Command;
use tower_http::services::ServeDir;
use urlencoding::encode;

use video_download_service::config::*;
use video_download_service::templates::*;

static DOWNLOAD_COMMAND: &str = "yt-dlp";

struct FileInfo {
    name: String,
    timestamp: SystemTime,
}

async fn display_index() -> Html<String> {
    Html(match TEMPLATE_ENGINE.render("index.html", &{}) {
        Ok(document) => document,
        Err(error) => error.to_string(),
    })
}

async fn display_download(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(match params.get("url") {
        Some(url) => {
            let value_map: HashMap<&str, String> = [("url", url.clone())].iter().cloned().collect();

            match handle_download(url).await {
                Ok(_) => TEMPLATE_ENGINE.render("finished.html", &value_map).unwrap(),
                Err(error) => {
                    println!("Error: {}", error);
                    let error_value_map: HashMap<&str, String> = [(
                        "error",
                        format!("Error downloading url: {}: {}", url, error),
                    )]
                    .iter()
                    .cloned()
                    .collect();

                    TEMPLATE_ENGINE
                        .render("error.html", &error_value_map)
                        .unwrap()
                }
            }
        }
        None => "Not found".to_string(),
    })
}

async fn handle_download(url: &str) -> Result<()> {
    // Get download directory
    // how to make sure we can change dir without affecting other clients?
    let download_dir = &CONFIG.download_dir;

    println!("Downloading url: '{}' to: '{}'", url, download_dir);

    let command = format!(
        "cd {}; {} --no-mtime '{}'",
        download_dir, DOWNLOAD_COMMAND, url
    );

    println!("With command: {}", command);

    // no-mtime: Use the download time for the timestamp so the listing order is based on download time
    match Command::new("sh").arg("-c").arg(command).output().await {
        Ok(output) => {
            let output_string = std::str::from_utf8(&output.stdout)?;
            let error_string = std::str::from_utf8(&output.stderr)?;
            if output.status.success() {
                println!("Output: {}", output_string);
                println!("Error output: {}", error_string);
                Ok(())
            } else {
                Err(anyhow!(
                    "yt-dlp failed: {}: {}",
                    output_string,
                    error_string
                ))
            }
        }
        Err(_) => Err(anyhow!("Error executing yt-dlp")),
    }
}

async fn dir_listing(directory: &str) -> Result<std::vec::Vec<FileInfo>> {
    let mut results: std::vec::Vec<FileInfo> = std::vec::Vec::new();

    // vector insert with compare func?

    let mut dir = read_dir(directory).await?;

    while let Some(entry) = dir.next_entry().await? {
        results.push(FileInfo {
            name: entry.file_name().to_str().unwrap().to_string(),
            timestamp: entry.metadata().await?.modified()?,
        });
    }

    // Sort by date so the listing reflects download time
    results.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    results.reverse();

    Ok(results)
}

async fn list_downloads() -> Html<String> {
    let mut html_list: String = String::new();

    html_list.push_str("<ul>\n");

    if let Ok(list) = dir_listing(&CONFIG.download_dir).await {
        for fileinfo in list {
            // Url encode the filename
            let encoded_file = encode(&fileinfo.name);

            html_list.push_str(
                format!(
                    "<li><a href=\"file/{}\">{}</a></li>\n",
                    encoded_file, fileinfo.name
                )
                .as_str(),
            );
        }
    }

    html_list.push_str("</ul>\n");

    let value_map: HashMap<&str, String> = [("filelist", html_list)].iter().cloned().collect();

    let document = TEMPLATE_ENGINE.render("filelist.html", &value_map).unwrap();

    Html(document)
}

#[tokio::main]
async fn main() {
    println!("Download dir: {}", &CONFIG.download_dir);

    let app = Router::new()
        .route("/", get(display_index))
        .route("/filelist", get(list_downloads))
        .route("/download", get(display_download))
        .nest_service("/file", ServeDir::new(&CONFIG.download_dir));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
