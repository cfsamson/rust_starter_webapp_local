use super::*;
use handlebars::Handlebars;
use rouille;
use rouille::{router, Request, Response};
use serde_json::json;
use std::process::Command;


const CSS: &str = include_str!("../www/assets/bootstrap-material-design.min.css");
const CSS_SITE: &str = include_str!("../www/assets/site.css");
const JS: &str = include_str!("../www/assets/bootstrap-material-design.min.js");

pub fn start(port: Option<u32>) -> Res<()> {
    let port = port.unwrap_or(8080);

    let ip = String::from("0.0.0.0:");
    let address = ip + port.to_string().as_str();
    let server_res = rouille::Server::new(&address, router);

    match server_res {
        Err(_) => {
            if (port + 1) > MAX_PORT {
                panic!(
                    "Didn't find any available ports in the range 8080 - 8090. Please free one!"
                );
            };
            start(Some(port + 1))
        }

        Ok(server) => {
            let start_command = String::from("start http://localhost:");
            let start_command = start_command + port.to_string().as_str();
            println!("{}", start_command);
            Command::new("cmd").args(&["/C", &start_command]).spawn()?;
            server.run();
            Ok(())
        }
    }
}

fn router(req: &Request) -> Response {
    let result = router!(req,
        (GET) (/) => {index_handler(&req)},
        (GET) (/css) => {css_handler(&req)},
        (GET) (/css/site) => {css_site_handler(&req)},
        (GET) (/js) => {js_handler(&req)},
        _ => {Ok(Response::text("Not found"))},
    );

    match result {
        Ok(res) => res,
        Err(e) => Response::text(format!("An error occurred {}", e)),
    }
}

fn index_handler(_: &Request) -> Res<Response> {
    let mut reg = Handlebars::new();
    let template = include_str!("../www/index.handlebars");
    let payload = reg.render_template(
        &template,
        &json!({"title": "Welcome", "body": "This is the body"}),
    )?;
    Ok(Response::html(payload))
}

fn css_handler(req: &Request) -> Res<Response> {
    Ok(Response::from_data("text/css", CSS))
}

fn css_site_handler(req: &Request) -> Res<Response> {
    Ok(Response::from_data("text/css", CSS_SITE))
}

fn js_handler(req: &Request) -> Res<Response> {
    Ok(Response::from_data("text/javascript", JS))
}

// fn query_db_example() -> Res<()> {
    
//     let conn = data::connect()?;

//     let registraions = data::Registration::find_registrations(&conn, "true", rusqlite::NO_PARAMS)?;
//     println!("{:?}", registraions);
//     for reg in registraions {
//         println!("date is: {}", reg.date()? + chrono::Duration::days(-7));
//     }

//     Ok(())
// }
