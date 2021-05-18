use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(|| {
        warp::reply::json(&json!({
            "apiversion": "1",
            "color": "",
            "head": "",
            "tail": "",
        }))
    });
    let start = warp::path("start")
        .and(warp::post())
        .map(|| warp::reply::with_status("", StatusCode::IM_A_TEAPOT));
    let end = warp::path("end")
        .and(warp::post())
        .map(|| warp::reply::with_status("", StatusCode::IM_A_TEAPOT));
    let get_move = warp::path("move")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|sent_move: Move| async move {
            // move logic
            Ok(warp::reply::json(&json!({
                "move": "up",
                "shout": ""
            }))) as Result<_, Rejection>
        });
    let routes = index
        .or(start)
        .or(end)
        .or(get_move);
    let port = std::env::var("PORT")
        .expect("PORT Environment Variable not set")
        .parse()
        .expect("PORT is not a valid port number");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

#[derive(Debug, Deserialize)]
struct Move {
    game: SentGame,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

#[derive(Debug, Deserialize)]
struct SentGame {
    id: String,
    timeout: u128,
}

#[derive(Debug, Deserialize)]
struct Board {
    height: u8,
    width: u8,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Battlesnake {
    id: String,
    name: String,
    health: u8,
    body: Vec<Coordinate>,
    latency: String,
    head: Coordinate,
    length: u16,
    shout: String,
}
#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Coordinate {
    x : u8,
    y : u8,
}
