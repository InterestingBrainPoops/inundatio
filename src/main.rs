mod types;
use dotenv::dotenv;
use serde_json::json;
use types::*;
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;
mod engine;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let index = warp::path::end().map(|| {
        warp::reply::json(&json!({
            "apiversion": "1",
            "color": "65bbc7",
            "head": "sand-worm",
            "tail": "rbc-necktie",
        }))
    });
    let start = warp::path("start")
        .and(warp::post())
        .map(|| warp::reply::with_status("", StatusCode::OK));
    let end = warp::path("end")
        .and(warp::post())
        .map(|| warp::reply::with_status("", StatusCode::OK));
    let get_move = warp::path("move")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|sent_move: Move| async move {
            let out_move;
            out_move = engine::getMove(sent_move);
            Ok(warp::reply::json(&json!({
                "move": out_move,
                "shout": "We've been trying to reach you concerning your vehicle's extended warranty."
            }))) as Result<_, Rejection>
        });
    let routes = index.or(start).or(end).or(get_move);
    let port = std::env::var("PORT")
        .expect("PORT Environment Variable not set")
        .parse()
        .expect("PORT is not an integer");
    println!("Listening on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
