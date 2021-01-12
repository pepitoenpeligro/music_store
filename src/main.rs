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

extern crate banana;

use banana::{App, Request, Response};

fn main() -> () {
    let mut a = App::new();

    fn this_handler(_:Request) -> Response {
        Response::ok_html("TESTTESTTEST".to_string())
    }

    fn another_handler(req:Request) -> Response {
        let name:String = match req.query_string.get(&"name".to_string()) {
            Some(n) => n.clone(),
            None => "anonymous".to_string(),
        };

        Response::ok_html(format!("Hello {}", name))
    }

    a.routes.insert("^/$", this_handler); 
    a.routes.insert("^/(?P<title>[^']+)$", another_handler);

    a.run("127.0.0.1:8080");
}