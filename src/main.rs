use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;
use warp::Filter;
use serde::{Deserialize, Serialize};

type DataStore = Arc<RwLock<BTreeMap<String, String>>>;

#[derive(Deserialize)]
struct Query {
    key: String,
}

#[derive(Serialize)]
struct Response {
    value: Option<String>,
}

async fn get_value(query: Query, store: DataStore) -> Result<impl warp::Reply, warp::Rejection> {
    let store = store.read().unwrap();
    let value = store.get(&query.key).cloned();
    Ok(warp::reply::json(&Response { value }))
}

async fn update_value(query: Query, value: String, store: DataStore) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = store.write().unwrap();
    store.insert(query.key, value);
    Ok(warp::reply::json(&"Update successful"))
}

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(BTreeMap::new()));
    let store_filter = warp::any().map(move || Arc::clone(&store));

    let get_route = warp::path("get")
        .and(warp::get())
        .and(warp::query::<Query>())
        .and(store_filter.clone())
        .and_then(|query: Query, store: DataStore| get_value(query, store));

    let update_route = warp::path("update")
        .and(warp::post())
        .and(warp::query::<Query>())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(|query: Query, value: String, store: DataStore| update_value(query, value, store));

    let routes = get_route.or(update_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}