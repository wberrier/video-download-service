# Features

* download video urls (via youtube-dl)
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
* [ ] template error messages
  * [ ] how to appease the borrorw checker with lifetimes and handlebars??

# Implementation

Looked at some rust web frameworks.  I think I'll try warp.  It's
tokio/hyper based.


