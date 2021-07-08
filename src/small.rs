use crate::types::{Coordinate, Direction};
use serde::Deserialize;
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct SmallBattleSnake {
    pub id: u8,
    pub health: u8,
    pub body: Vec<Coordinate>,
    pub head: Coordinate,
    pub length: u16,
}
impl SmallBattleSnake {
    pub fn get_moves(&self) -> Vec<(Direction, u8)> {
        let out = vec![
            (Direction::Up, self.id),
            (Direction::Down, self.id),
            (Direction::Left, self.id),
            (Direction::Right, self.id),
        ];
        out
    }
    pub fn new(id: u8, health: u8, body: &Vec<Coordinate>) -> SmallBattleSnake {
        SmallBattleSnake {
            id,
            health,
            body: body.clone(),
            head: body[0],
            length: body.len() as u16,
        }
    }
    pub fn make_move(&mut self, move_to_make: Direction) -> Coordinate {
        self.health -= 1; // decrement the health
        let add = match move_to_make {
            Direction::Up => Coordinate::new(0, 1),
            Direction::Down => Coordinate::new(0, -1),
            Direction::Left => Coordinate::new(-1, 0),
            Direction::Right => Coordinate::new(1, 0),
        };
        self.head += add;
        self.body.insert(0, self.head);
        // println!("{}", snake.body.len());
        match self.body.pop() {
            Some(x) => x,
            None => panic!("snakes were at length zero. This shouldn't happen."),
        }
    }
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SmallMove {
    pub turn: u32,
    pub board: SmallBoard,
    pub you: SmallBattleSnake,
}
impl SmallMove {
    pub fn new() -> SmallMove {
        SmallMove {
            turn: 0,
            board: SmallBoard::new(),
            you: SmallBattleSnake::new(0, 0, &vec![Coordinate::new(0, 0)]),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SmallBoard {
    pub height: i8,
    pub width: i8,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<SmallBattleSnake>,
    // pub dead: Vec<String>, // a vector of the dead ids.
}
impl SmallBoard {
    pub fn new() -> SmallBoard {
        SmallBoard {
            height: 0,
            width: 0,
            food: vec![],
            hazards: vec![],
            snakes: vec![],
        }
    }
}
