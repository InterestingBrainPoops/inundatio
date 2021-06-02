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
    pub dead: Vec<String>, // a vector of the dead ids.
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
    pub died: Vec<String>, // the ids of the snakes that died during this turn
    pub tails: Vec<(String, Coordinate)>, // the tails of the snakes that were removed during this turn
    pub eaten_food: Vec<Coordinate>, // the positions of the food that were eaten during this turn ( if any )
}
impl Move {
    // TODO: THIS HAS A LOT OF CLONES. probably not a good idea because memory space / usage will go up fast. 
    // at least i think
    pub fn make_move(&mut self, moves: &Vec<SnakeMove>, turn: bool) -> Delta {
        let mut out = Delta {
            died: vec![],
            tails: vec![],
            eaten_food: vec![],
        };
        // the following for loop removes all tails, and also moves all snakes within the given moves.
        for snake in &mut self.board.snakes {
            // checks that the snake is alive.
            if !self.board.dead.contains(&snake.id) {
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
                // checks if the head is on any food, and if it is, then it removes the food, and gives the snake max health.
                match self.board.food.iter().position(|&r| r == snake.head) {
                    Some(index) => {
                        out.eaten_food.push(self.board.food.remove(index)); // removes the food at the given index.
                        snake.health = 100;
                        snake.body.push(snake.body[snake.body.len() - 1]); // basically dupes the tail.
                    }
                    None => {}
                }
                if snake.id.eq(&self.you.id) {
                    self.you = snake.clone();
                }
            }
        }
        // following kills snakes if they are:
        //   out of bounds
        //   out of health (<= 0)
        //   head to body collision
        //   head to head collision
        for snake in &self.board.snakes {
            if  !self.board.dead.contains(&snake.id) {
                if snake.health <= 0 {
                    self.board.dead.push(snake.id.clone());
                    out.died.push(snake.id.clone());
                    // no health
                } else if snake.head.x < 0
                    || snake.head.y < 0
                    || snake.head.x >= self.board.width
                    || snake.head.y >= self.board.height
                {
                    self.board.dead.push(snake.id.clone());
                    out.died.push(snake.id.clone());
                    // out of bounds
                }else{
                    for opp in &self.board.snakes {
                        
                        // the following is to check if my head is within their body.
                        if !self.board.dead.contains(&opp.id) && opp.body.contains(&snake.head)  {
                            self.board.dead.push(snake.id.clone());
                            out.died.push(snake.id.clone());
                            break;
                        }
                    }
                }
            }
            
        }
        out
    }
    pub fn unmake_move ( &mut self, delta : &Delta) {
        // add tails back to snakes
        // remove the heads
        // increase all snake health by 1
        // revive all killed snakes
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
