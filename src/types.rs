use crate::cartprod;
use serde::Deserialize;
use std::num::ParseIntError;
use std::ops;
use std::str::FromStr;
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Move {
    pub game: SentGame,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SentGame {
    pub id: String,
    pub timeout: u128,
}
#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Board {
    pub height: i8,
    pub width: i8,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<Battlesnake>,
    // pub dead: Vec<String>, // a vector of the dead ids.
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
    pub eaten_food: Vec<(String,u8, Coordinate)>, // (id, previous health, where the food was)
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State {
    pub state: Move,       // current state
    pub dead: Vec<String>, // ids
}
impl State {
    // TODO: THIS HAS A LOT OF CLONES. probably not a good idea because memory space / usage will go up fast.
    // at least i think
    fn make_move(&mut self, moves: &Vec<SnakeMove>) -> Delta {
        let mut out = Delta {
            died: vec![],
            tails: vec![],
            eaten_food: vec![],
        };
        // the following for loop removes all tails, and also moves all snakes within the given moves.
        for snake in &mut self.state.board.snakes {
            // checks that the snake is alive.
            if !self.dead.contains(&snake.id) {
                
                for snakes_move in moves {
                    // this entire block here just moves the snakes in the direction they chose
                    if snake.id == snakes_move.id {
                        // basically if it matches the id.
                        snake.health -= 1; // decrement the health
                        let add = match snakes_move.snake_move {
                            Direction::Up => Coordinate::new(0, 1),
                            Direction::Down => Coordinate::new(0, -1),
                            Direction::Left => Coordinate::new(-1, 0),
                            Direction::Right => Coordinate::new(1, 0),
                        };
                        snake.head += add;
                        snake.body.insert(0, snake.head);
                        // println!("{}", snake.body.len());
                        match snake.body.pop() {
                            Some(x) => {
                                out.tails.push((snake.id.clone(), x));
                            }
                            None => panic!("snakes were at length zero. This shouldn't happen."),
                        }
                        // checks if the head is on any food, and if it is, then it removes the food, and gives the snake max health.
                        match self.state.board.food.iter().position(|&r| r == snake.head) {
                            Some(index) => {
                                out.eaten_food.push((snake.id.clone(),snake.health,  self.state.board.food.remove(index))); // removes the food at the given index.
                                snake.health = 100;
                                snake.body.push(snake.body[snake.body.len() - 1]); // basically dupes the tail.
                                snake.length += 1;
                            }
                            None => {}
                        }
        
                        if snake.id.eq(&self.state.you.id) {
                            self.state.you = snake.clone();
                        }
                    }
                }
                
            }
        }
        // following kills snakes if they are:
        //   out of bounds
        //   out of health (<= 0)
        //   head to body collision
        //   head to head collision
        for snake in &self.state.board.snakes {
            if !self.dead.contains(&snake.id) {
                if snake.health <= 0 {
                    // out of health

                    out.died.push(snake.id.clone());
                    // no health
                } else if snake.head.x < 0
                    || snake.head.y < 0
                    || snake.head.x >= self.state.board.width
                    || snake.head.y >= self.state.board.height
                {
                    // out of bounds

                    out.died.push(snake.id.clone());
                    // out of bounds
                } else if snake.body[1..].contains(&snake.head) {
                    out.died.push(snake.id.clone());
                } else {
                    for opp in &self.state.board.snakes {
                        if !self.dead.contains(&opp.id) {
                            // another battlesnake collision
                            if opp.body[1..].contains(&snake.head) {
                                out.died.push(snake.id.clone());
                                break;
                            } else if opp.head == snake.head
                                && snake.length <= opp.length
                                && snake.id != opp.id
                            {
                                // head to head and losing.
                                out.died.push(snake.id.clone());
                                break;
                            }
                        }
                    }
                }
            }
        }

        self.dead.append(&mut out.died.clone());
        
        out
    }
    fn unmake_move(&mut self, delta: &Delta) {
        // revive all killed snakes
        self.dead.retain(|x| !delta.died.contains(x));
        
        // add tails back to snakes
        // and remove all heads
        for food in &delta.eaten_food {
            for snake in &mut self.state.board.snakes {
                if food.0 == snake.id {
                    self.state.board.food.push(food.2);
                    snake.health = food.1;
                    snake.body.pop();
                    snake.length -= 1;
                }
            }
        }
        for tail in &delta.tails {
            for snake in &mut self.state.board.snakes {
                if tail.0 == snake.id {
                    snake.body.push(tail.1);
                    snake.health += 1;
                    snake.body.remove(0);
                    snake.head = snake.body[0];
                    if snake.id == self.state.you.id {
                        self.state.you = snake.clone();
                    }
                }
            }
        }
    }

    /// Depth is how far to search
    /// maximizing is whether the function is supposed to be maximizing or minimizing.
    fn minimax(
        &mut self,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
        static_eval: &dyn Fn(&Move, &Vec<String>) -> i32,
        count : &mut i32,
    ) -> (i32, i32, i32) {
        
        if depth == 0 || self.dead.contains(&self.state.you.id) || self.state.board.snakes.len() - self.dead.len()  == 1 {
            *count += 1;
            if(self.dead.contains(&self.state.you.id)){
                return (i32::MIN,alpha,beta);
            }else if self.state.board.snakes.len() - self.dead.len() == 1{
                return (i32::MAX, alpha, beta);
            }
            return (static_eval(&self.state, &self.dead), alpha, beta);
        }
        if maximizing {
            let mut value = i32::MIN;
            for current_move in &self.state.you.get_moves() {
                let delta = self.make_move(&vec![current_move.clone()]);
                value = i32::max(
                    value,
                    self.minimax(depth - 1, alpha, beta, !maximizing, static_eval,count)
                        .0,
                );
                self.unmake_move(&delta);
                if value >= beta {
                    break; // beta cutoff
                }
                alpha = i32::max(alpha, value);
            }
            return (value, alpha, beta);
        } else {
            let mut value = i32::MAX;
            for current_move in &self.get_moves() {
                let delta = self.make_move(current_move);
                value = i32::min(
                    value,
                    self.minimax(depth - 1, alpha, beta, !maximizing, static_eval,count)
                        .0,
                );
                self.unmake_move(&delta);
                if value <= alpha {
                    break;
                }
                beta = i32::min(beta, value);
            }
            return (value, alpha, beta);
        }
    }
    /// It will return a 2D array of moves for the opposing team.
    fn get_moves(&self) -> Vec<Vec<SnakeMove>> {
        let mut out: Vec<Vec<SnakeMove>> = vec![];
        for x in (&self.state.board.snakes)
            .into_iter()
            .filter(|x| x.id != self.state.you.id)
        {
            out.push(x.get_moves());
        }
        let x = cartprod::cartesian_product(out);

        x
    }
    pub fn get_best(&mut self, static_eval: &dyn Fn(&Move, &Vec<String>) -> i32) -> (Direction, &str, i32) {
        let mut out = vec![(Direction::Up,"up", 0),(Direction::Down, "down", 0),(Direction::Left, "left",0),(Direction::Right, "right",0)];
        let mut  alpha= i32::MIN;let mut  beta = i32::MAX;
        for x in &mut out {
            let delta = self.make_move(&vec![SnakeMove::new(x.0,self.state.you.id.clone())]);
            let mut count = 0;
            let a = self.minimax(5,alpha,beta,false, static_eval, &mut count);
            println!("Total number of end states reached: {}", count);
            self.unmake_move(&delta);
            x.2 = a.0;
            
            alpha = a.1;
            beta = a.2;
        }
        // assert_eq!(e, *self);
        let mut biggest = &out[0];
        for x in &out[1..] {
            if biggest.2 < x.2 {
                biggest = x;
            }
        }
        *biggest
        
    }
}

impl Battlesnake {
    pub fn get_moves(&self) -> Vec<SnakeMove> {
        let out = vec![
            SnakeMove::new(Direction::Up, self.id.clone()),
            SnakeMove::new(Direction::Down, self.id.clone()),
            SnakeMove::new(Direction::Left, self.id.clone()),
            SnakeMove::new(Direction::Right, self.id.clone()),
        ];
        out
    }
}
impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            _ => {
                panic!("things happened")
            }
        }
    }
}
#[derive(Debug, Deserialize, Eq, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct SnakeMove {
    pub snake_move: Direction,
    pub id: String,
}
impl SnakeMove {
    pub fn new(dir: Direction, id: String) -> SnakeMove {
        SnakeMove {
            snake_move: dir,
            id,
        }
    }
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
