use std::ops;

use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Move {
    pub game: SentGame,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}
#[derive(Debug, Deserialize, Clone)]
pub struct SentGame {
    pub id: String,
    pub timeout: u128,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Board {
    pub height: i8,
    pub width: i8,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<Battlesnake>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Delta {
    pub dead: Vec<Battlesnake>, // the snakes that died during this turn
    pub tails: Vec<(String, Coordinate)>, // the tails of the snakes that were removed during this turn
    pub eaten_food: Vec<Coordinate>, // the positions of the food that were eaten during this turn ( if any )
}
impl Move {
    pub fn make_move(&mut self, moves: &Vec<SnakeMove>, turn: bool) -> Delta {
        let mut out = Delta {
            dead: vec![],
            tails: vec![],
            eaten_food: vec![],
        };
        let mut all_alive: Vec<Battlesnake> = vec![];
        for snake in &mut self.board.snakes {
            // each snake
            snake.health -= 1; // decrement the health
            for snakes_move in moves {
                // this entire block here just moves the snakes in the direction they chose
                if snake.id == snakes_move.id {
                    // basically if it matches the id.
                    let add = match snakes_move.snake_move.as_str() {
                        "up" => Coordinate::new(0, 1),
                        "down" => Coordinate::new(0, -1),
                        "left" => Coordinate::new(-1, 0),
                        "right" => Coordinate::new(1, 0),
                        _ => {
                            panic!("A move was not UDLR");
                        }
                    };
                    snake.head += add;
                    snake.body.insert(0, snake.head);
                    match snake.body.pop() {
                        Some(x) => {
                            out.tails.push((snake.id.clone(), x));
                        }
                        None => panic!("snakes were at length zero. This shouldn't happen."),
                    }
                }
            }
            // checks if the head is on any food, and if it is, then it removes it.
            match self.board.food.iter().position(|&r| r == snake.head) {
                Some(index) => {
                    out.eaten_food.push(self.board.food.remove(index)); // removes the food at the given index.
                    snake.health = 100;
                    snake.body.push(snake.body[snake.body.len() - 1]); // basically dupes the tail.
                }
                None => {}
            }
        }
		for snake in &self.board.snakes {
			let mut alive = true; // basically whether or not this snake is dead.
            if snake.health <= 0 {
                // no health
                alive = false;
            } else if snake.head.x < 0
                || snake.head.y < 0
                || snake.head.x >= self.board.width
                || snake.head.y >= self.board.height
            {
                // out of bounds
                alive = false;
            }
			if alive {
				for x in &self.board.snakes {
                    for y in &x.body[1..] {
                        if *y == snake.head {
                            alive = false;
                        }
                    }
                }
                alive = true;
            }
            if alive {
                all_alive.push(snake.clone());
            }
		}
        out
    }
}
#[derive(Debug, Deserialize, Eq, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct SnakeMove {
    pub snake_move: String,
    pub id: String,
}

impl ops::AddAssign<Coordinate> for Coordinate {
    fn add_assign(&mut self, rhs: Coordinate) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Coordinate {
    pub fn new(x: i8, y: i8) -> Self {
        Coordinate { x, y }
    }
}
