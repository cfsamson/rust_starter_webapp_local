# Shell project for small local apps with a web interface

## What is it
This is just a basic shell app for making small **local** applications running on one computer or on a loca network. Think of this as the same kind of interface as pgadmin and the like uses. This is intended to work offline, but if you use jQuery remember to include that since we get that through a CDN.

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

The web server. When the server finds a available port to use (by default it tries in the range 8080-8090 before it stops), we start the web browser and points it to our webpage. **This is only set up for windows. We'll need to add a `cfg` flag seperating the shell commands between windows and osx/linux it you want it to work there**. Look here for further info: [https://doc.rust-lang.org/beta/std/process/struct.Command.html][link1]

### data.rs

Setup and functions for the database. The design here is very simple and ment to be adjusted according to the size of the project. Either way it's easy to change it to a repository pattern. NB! Be careful. One of the methods invites to take "where" clause from the frontend. This is not the intention. This is just for developer convenience. Anyways, the data here is not secured by any means and is intended to live on the users computer. If you need to syncronize it you should make a service that syncs the database to a remote one.

### sql_files/

Sql scripts, especially for creating/dropping the database. 

## Intended use
Create new routes in the web module. Edit `site.css` to add more custom styles. Javascript is intended to go in the bottom of the html pages.

[link1]:https://doc.rust-lang.org/beta/std/process/struct.Command.html