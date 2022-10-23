# Features

* download video urls (via yt-dlp)
* place videos in certain locations (read only), with ip/name subdirs
* log of videos downloaded
* only keep n videos per ip? (but store logs indefinitely)

# Steps

* [x] Listen on port
* [x] hello world
* [x] config file mechanism
* [x] Accept form
* [x] parse form info
* [x] provide download feedback (necessary?)
* [x] done page
* [x] template error messages
* [x] file browsing option (instead of download url to point to local
      path)
* [x] systemd file
* [ ] option for download audio only
* [ ] download subdirectory based on ip
  * https://docs.rs/warp/0.2.5/warp/filters/addr/fn.remote.html
* [ ] log all requests under subdirectory
  * [ ] find good logging crate
* [ ] only keep "last n" for each ip
* [ ] put cargo/git version into binary to display on webpage
* [ ] allow specifying path to downloader program
* [ ] include Dockerfile for easy deployment

# Implementation

Looked at some rust web frameworks.  I think I'll try warp.  It's
tokio/hyper based.


