# Features

* download video urls (via youtube-dl)
* place videos in certain locations (read only), with ip/name subdirs
* log of videos downloaded
* only keep n videos per ip? (but store logs indefinitely)

# Steps

* Listen on port
* hello world
* config file mechanism
* Accept form
* parse form info
* provide download feedback (necessary?)
* done page

# Implementation

Looked at some rust web frameworks.  I think I'll try warp.  It's
tokio/hyper based.


