mod cartprod;
mod engine;
mod small;
mod types;
// use crate::{
//     small::{SmallBattleSnake, Status::*},
//     types::{Direction, State},
// };
// use engine::eval;
// use small::{SmallBoard, SmallMove};
// use types::Coordinate;

use dotenv::dotenv;
use serde_json::json;
use std::time::Instant;
use types::*;
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let index = warp::path::end().map(|| {
        warp::reply::json(&json!({
            "apiversion": "1",
            "color": "#65bbc7",
            "head": "sand-worm",
            "tail": "rbc-necktie",
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
            println!("GOT MOVE");
            let mut pstate = State{state:sent_move.into_small()};
            let mut state = State{state:sent_move.into_small()};
            println!("{:?}", state);
            let out_move;
            println!("{:?}", pstate.state.you.get_moves(&pstate.state.board));
            println!("{}", pstate.perft(1, (Direction::Up, 5), true));
            out_move = state.get_best(&engine::eval, 9);

            Ok(warp::reply::json(&json!({
                "move": out_move.1,
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

// test thing, ignore.
// fn main() {
//     let mut x = State {
//         state: SmallMove {
//             turn: 43,
//             board: SmallBoard {
//                 height: 11,
//                 width: 11,
//                 food: vec![Coordinate { x: 0, y: 2 }],
//                 hazards: vec![],
//                 snakes: vec![
//                     SmallBattleSnake {
//                         id: 1,
//                         health: 59,
//                         body: vec![
//                             Coordinate { x: 0, y: 1 },
//                             Coordinate { x: 1, y: 1 },
//                             Coordinate { x: 2, y: 1 },
//                             Coordinate { x: 2, y: 0 },
//                         ],
//                         head: Coordinate { x: 0, y: 1 },
//                         length: 4,
//                         status: Alive,
//                     },
//                     SmallBattleSnake {
//                         id: 2,
//                         health: 90,
//                         body: vec![
//                             Coordinate { x: 1, y: 2 },
//                             Coordinate { x: 1, y: 3 },
//                             Coordinate { x: 2, y: 3 },
//                             Coordinate { x: 2, y: 4 },
//                             Coordinate { x: 3, y: 4 },
//                             Coordinate { x: 3, y: 5 },
//                             Coordinate { x: 4, y: 5 },
//                             Coordinate { x: 4, y: 6 },
//                         ],
//                         head: Coordinate { x: 1, y: 2 },
//                         length: 8,
//                         status: Alive,
//                     },
//                 ],
//             },
//             you: SmallBattleSnake {
//                 id: 1,
//                 health: 59,
//                 body: vec![
//                     Coordinate { x: 0, y: 1 },
//                     Coordinate { x: 1, y: 1 },
//                     Coordinate { x: 2, y: 1 },
//                     Coordinate { x: 2, y: 0 },
//                 ],
//                 head: Coordinate { x: 0, y: 1 },
//                 length: 4,
//                 status: Alive,
//             },
//         },
//     };
//     x.make_move(&vec![(Direction::Down, 1), (Direction::Left, 2)]);
//     x.make_move(&vec![(Direction::Right, 1), (Direction::Up, 2)]);
//     x.get_best(&eval, 8);
// }
