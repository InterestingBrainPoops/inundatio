use std::ops;

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Move {
    pub game: SentGame,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Debug, Deserialize)]
pub struct SentGame {
    pub id: String,
    pub timeout: u128,
}

#[derive(Debug, Deserialize)]
pub struct Board {
    pub height: i8,
    pub width: i8,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<Battlesnake>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: u8,
    pub body: Vec<Coordinate>,
    pub latency: String,
    pub head: Coordinate,
    pub length: u16,
    pub shout: String,
}
#[derive(Debug, Deserialize, Eq, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}
impl ops::AddAssign<Coordinate> for Coordinate {
    fn add_assign(&mut self, rhs: Coordinate) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
