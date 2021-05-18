use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Move {
    game: SentGame,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

#[derive(Debug, Deserialize)]
pub struct SentGame {
    id: String,
    timeout: u128,
}

#[derive(Debug, Deserialize)]
pub struct Board {
    height: u8,
    width: u8,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Battlesnake {
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
pub struct Coordinate {
    x: u8,
    y: u8,
}
