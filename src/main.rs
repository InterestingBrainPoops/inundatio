mod cartprod;
mod engine;
mod small;
mod types;
use crate::{small::SmallBattleSnake, types::State};
use engine::eval;
use small::{SmallBoard, SmallMove};
use types::Coordinate;
// use dotenv::dotenv;
// use serde_json::json;
// use std::time::Instant;
// use types::*;
// use warp::http::StatusCode;
// use warp::Filter;
// use warp::Rejection;
// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//     let index = warp::path::end().map(|| {
//         warp::reply::json(&json!({
//             "apiversion": "1",
//             "color": "#65bbc7",
//             "head": "sand-worm",
//             "tail": "rbc-necktie",
//         }))
//     });
//     let start = warp::path("start")
//         .and(warp::post())
//         .map(|| warp::reply::with_status("", StatusCode::IM_A_TEAPOT));
//     let end = warp::path("end")
//         .and(warp::post())
//         .map(|| warp::reply::with_status("", StatusCode::IM_A_TEAPOT));
//     let get_move = warp::path("move")
//         .and(warp::post())
//         .and(warp::body::json())
//         .and_then(|sent_move: Move| async move {
//             let start = Instant::now();
//             println!("GOT MOVE");
//             let mut state = State{state:sent_move.into_small(), dead: vec![]};
//             let out_move;
//             out_move = state.get_best(&engine::eval);
//             println!("Turn: {}, ToMove: {:?}, Score: {}",sent_move.turn , out_move.0, out_move.2);
//             println!("took me {:?}",  start.elapsed());
//             Ok(warp::reply::json(&json!({
//                 "move": out_move.1,
//                 "shout": "We've been trying to reach you concerning your vehicle's extended warranty."
//             }))) as Result<_, Rejection>
//         });
//     let routes = index.or(start).or(end).or(get_move);
//     let port = std::env::var("PORT")
//         .expect("PORT Environment Variable not set")
//         .parse()
//         .expect("PORT is not an integer");
//     println!("Listening on port {}", port);
//     warp::serve(routes).run(([0, 0, 0, 0], port)).await;
// }
fn main() {
    let mut x = State {
        state: SmallMove {
            turn: 9,
            board: SmallBoard {
                height: 11,
                width: 11,
                food: vec![Coordinate { x: 2, y: 6 }],
                hazards: vec![],
                snakes: vec![
                    SmallBattleSnake {
                        id: 1,
                        health: 91,
                        body: vec![
                            Coordinate { x: 2, y: 7 },
                            Coordinate { x: 2, y: 8 },
                            Coordinate { x: 2, y: 9 },
                        ],
                        head: Coordinate { x: 2, y: 7 },
                        length: 3,
                    },
                    SmallBattleSnake {
                        id: 2,
                        health: 99,
                        body: vec![
                            Coordinate { x: 5, y: 6 },
                            Coordinate { x: 5, y: 5 },
                            Coordinate { x: 4, y: 5 },
                            Coordinate { x: 4, y: 4 },
                            Coordinate { x: 4, y: 3 },
                        ],
                        head: Coordinate { x: 5, y: 6 },
                        length: 5,
                    },
                    SmallBattleSnake {
                        id: 3,
                        health: 93,
                        body: vec![
                            Coordinate { x: 6, y: 7 },
                            Coordinate { x: 7, y: 7 },
                            Coordinate { x: 8, y: 7 },
                            Coordinate { x: 8, y: 8 },
                        ],
                        head: Coordinate { x: 6, y: 7 },
                        length: 4,
                    },
                ],
            },
            you: SmallBattleSnake {
                id: 1,
                health: 91,
                body: vec![
                    Coordinate { x: 2, y: 7 },
                    Coordinate { x: 2, y: 8 },
                    Coordinate { x: 2, y: 9 },
                ],
                head: Coordinate { x: 2, y: 7 },
                length: 3,
            },
        },
        dead: vec![],
    };
    println!("{:?}", x.get_best(&eval));
}
