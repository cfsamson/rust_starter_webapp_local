# Shell project for small local apps with a web interface

## What it is
This is just a basic shell app for making small **local** applications running on one computer or on a local network. A major goal is to only have to distribute **one** file to users. Think of this as the same kind of interface as pgadmin and the like uses. This is intended to work offline, but if you use jQuery remember to include that since we get that through a CDN.

## Running it

Clone this repository. Run `cargo run` or `cargo build --release` and run the executable. On Windows you should get a web browser to open at the right address. It should work on Windows, OSX and Linux but if it's only tested on Windows and OSX. If it doesn't work on your platform edit `start()` in `src/web.rs` and add Command suitable for your operating system: 

```rust
Command::new("cmd").args(&["/C", &start_command]).spawn()?;
```

## Why
Crating user interfaces like this allows for a lot of reuse and possible scalability in addition this can be used from all devices with a browser even though the application itself must be run on a computer.

## How
We're using the following libraries:

* rusqlite: File based database. We use the "bundled" feature.
* rouille: simple synchronous web framework
* handlebars_rust: templating for the web interface
* chrono: time
* bootstrap: minified versions hard coded into the binary for easier UI styling
* jquery: included via CDN (but for offline use we need to remember to include it the same way we do with bootstrap)

## project layout

### main.rs

Main application entry point. Kept to a minimum. Hooks up web server.

### web.rs

The web server. When the server finds a available port to use (by default it tries in the range 8080-8090 before it stops), we start the web browser and points it to our webpage. Look here for further info: [https://doc.rust-lang.org/beta/std/process/struct.Command.html][link1]

### data.rs

Setup and functions for the database. The design here is very simple and ment to be adjusted according to the size of the project. Either way it's easy to change it to a repository pattern. NB! Be careful. One of the methods invites to take "where" clause from the frontend. This is not the intention. This is just for developer convenience. Anyways, the data here is not secured by any means and is intended to live on the users computer. If you need to syncronize it you should make a service that syncs the database to a remote one.

### sql_files/

Sql scripts, especially for creating/dropping the database. 

## Intended use
Create new routes in the web module. Edit `site.css` to add more custom styles. Javascript is intended to go in the bottom of the html pages.

## TODO

* Add a better logging solution
* Consider an external config file/env etc. for easier configuration (ip, port ranges etc) - however the ease of distributing only one file is the major goal.
* Organize the data access module to a repository pattern for a better boilerplate start
* Bootstrap gives an error not finding popper.js, probably some registration needed but I haven't figured that out yet
* Suggestions?

## How it looks

![Preview1](https://user-images.githubusercontent.com/8337848/52183756-8d301280-280b-11e9-875b-eaddea9f8cb2.png)

[link1]:https://doc.rust-lang.org/beta/std/process/struct.Command.html
