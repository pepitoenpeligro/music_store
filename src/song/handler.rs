use warp::{http};
use std::collections::HashMap;
use super::model;

pub async fn update_list_handler(
    song: super::model::Song,
    store: super::model::Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.list.write().insert(song.title, song.duration);

        Ok(warp::reply::with_status(
            "Added song to list",
            http::StatusCode::CREATED,
        ))
}

pub async fn delete_list_handler(
    song_id: model::Id,
    store: model::Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.list.write().remove(&song_id.title);

        Ok(warp::reply::with_status(
            "Removed song from list",
            http::StatusCode::OK,
        ))
}

pub async fn get_list_handler(
    store: model::Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut result = HashMap::new();
        let r = store.list.read();

        for (key,value) in r.iter() {
            result.insert(key, value);
        }

        Ok(warp::reply::json(
            &result
        ))
}