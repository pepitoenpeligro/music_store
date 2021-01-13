// use warp::{Filter};

// mod song;
// mod json;

// use crate::song::model::{Store};
// use crate::song::handler::{update_list_handler,get_list_handler,delete_list_handler};
// use crate::json::helpers::{post_json,delete_json};


// #[tokio::main]
// async fn main() {
//     let store = Store::new();
//     let store_filter = warp::any().map(move || store.clone());

//     let add_items = warp::post()
//         .and(warp::path("songs"))
//         .and(warp::path::end()).and(post_json())
//         .and(store_filter.clone())
//         .and_then(update_list_handler);

//     let get_items = warp::get()
//         .and(warp::path("songs"))
//         .and(warp::path::end())
//         .and(store_filter.clone())
//         .and_then(get_list_handler);

//     let delete_item = warp::delete()
//         .and(warp::path("songs"))
//         .and(warp::path::end())
//         .and(delete_json())
//         .and(store_filter.clone())
//         .and_then(delete_list_handler);

//     let update_item = warp::put()
//         .and(warp::path("songs"))
//         .and(warp::path::end())
//         .and(post_json())
//         .and(store_filter.clone())
//         .and_then(update_list_handler);

//     let routes = add_items.or(get_items).or(delete_item).or(update_item);

//     warp::serve(routes)
//         .run(([0, 0, 0, 0], 8000))
//         .await;
// }


use actix_web::{server, App, HttpRequest, Responder};
use std::env;

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run();
}