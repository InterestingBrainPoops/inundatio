use crate::cartprod;
use crate::move_table::MoveTable;
use crate::small::SmallBattleSnake;
use crate::small::SmallMove;
use crate::small::Status;
use serde::Deserialize;
use std::num::ParseIntError;
use std::ops;
use std::str::FromStr;
use std::time::Instant;
use std::u128;
use tinyvec::ArrayVec;

pub const TABLE_SIZE: usize = 2000000; // At 12 bytes per entry, this causes a size of 24 megabytes.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Weights(pub i32, pub i32, pub i32, pub i32);

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Move {
    pub game: SentGame,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}
impl Move {
    pub fn into_small(&self) -> SmallMove {
        let mut count = 1;
        let mut out = SmallMove::new();
        out.turn = self.turn;
        for x in &self.board.snakes {
            out.board
                .snakes
                .push(SmallBattleSnake::new(count, x.health, &x.body));
            if x.id == self.you.id {
                out.you = out.board.snakes.last().unwrap().clone();
            }
            count += 1;
        }
        out.board.food = self.board.food.clone();
        out.board.width = self.board.width;
        out.board.height = self.board.height;
        out
    }
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SentGame {
    pub id: String,
    pub timeout: u128,
}
#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq)]
pub enum Direction {
    Up = 0,
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
    pub died: Vec<u8>, // the ids of the snakes that died during this turn
    pub tails: Vec<(u8, Coordinate)>, // the tails of the snakes that were removed during this turn
    pub eaten_food: Vec<(u8, u8, Coordinate)>, // (id, previous health, where the food was)
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State {
    pub state: SmallMove, // current state
    pub weights: Weights,
    pub move_table: MoveTable,
    pub zobrist: u64,
    pub current_depth: u8,
    pub tt_hits: (u64, u64), // (total times i pinged the table, total number of legal moves)
}
impl State {
    // TODO: THIS HAS A LOT OF CLONES. probably not a good idea because memory space / usage will go up fast.
    // at least i think
    pub fn make_move(&mut self, moves: &ArrayVec<[(Direction, u8); 2]>) -> Delta {
        let mut out = Delta {
            died: vec![],
            tails: vec![],
            eaten_food: vec![],
        };
        out.tails = self.move_snakes(moves);
        out.eaten_food = self.maybe_feed_snakes();
        out.died = self.maybe_eliminate_snakes();

        for x in &self.state.board.snakes {
            if x.id == self.state.you.id {
                self.state.you = x.clone();
                break;
            }
        }
        self.update_zobrist(moves);
        out
    }
    /// Updates the zobrist hash given the moves.
    /// Its symmetrical , so calling it twice will have the same effect as not calling it at all.
    fn update_zobrist(&mut self, moves: &ArrayVec<[(Direction, u8); 2]>) {
        // turn moves into a u8 that uses the encoding decided on
        // player 0 goes first (in binary (basically on the left.))
        // shift it into the current hash.
        // In the hash, the first move goes all the way to the right.

        let p1 = ((self.state.board.snakes[0].head.x as u16) << 8)
            | (self.state.board.snakes[0].head.y as u16);
        let p2 = ((self.state.board.snakes[1].head.x as u16) << 8)
            | (self.state.board.snakes[1].head.y as u16);
        let combined = ((p1 as u64) << 32) ^ (p2 as u64);

        self.zobrist ^= combined;
    }
    fn move_snakes(&mut self, moves: &ArrayVec<[(Direction, u8); 2]>) -> Vec<(u8, Coordinate)> {
        let mut out: Vec<(u8, Coordinate)> = vec![];
        for snake in &mut self.state.board.snakes {
            for snake_move in moves {
                if snake.id == snake_move.1 && snake.status == Status::Alive {
                    out.push((snake.id, snake.make_move(snake_move.0)));
                }
            }
        }
        out
    }
    fn maybe_feed_snakes(&mut self) -> Vec<(u8, u8, Coordinate)> {
        let mut out: Vec<(u8, u8, Coordinate)> = vec![];
        for snake in &mut self.state.board.snakes {
            if snake.status == Status::Alive {
                for food in &self.state.board.food {
                    if snake.head == *food {
                        out.push((snake.id, snake.health, *food));

                        snake.health = 100;
                        snake.length += 1;
                        snake
                            .body
                            .push(*snake.body.last().expect("snake was at length 0"));
                    }
                }
            }
        }
        self.state
            .board
            .food
            .retain(|food| !out.iter().any(|eaten| eaten.2 == *food));
        out
    }

    fn maybe_eliminate_snakes(&mut self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        for snake in &mut self.state.board.snakes {
            if snake.status == Status::Dead {
                continue;
            }
            if snake.health == 0 {
                snake.status = Status::Dead;
                out.push(snake.id);
                continue;
            }
            if snake.is_out_of_bounds(self.state.board.width, self.state.board.height) {
                snake.status = Status::Dead;
                out.push(snake.id);
                continue;
            }

            // head to head and self collisions
            if snake.collision_with(snake) {
                snake.status = Status::Dead;
                out.push(snake.id);
                continue;
            }
        }

        let mut collision_eliminations: Vec<u8> = vec![];
        for snake in &self.state.board.snakes {
            let mut collide = false;
            for other in &self.state.board.snakes {
                if other.id != snake.id
                    && other.status != Status::Dead
                    && snake.collision_with(other)
                {
                    collision_eliminations.push(snake.id);
                    collide = true;
                    break;
                }
            }
            if collide {
                continue;
            }
            collide = false;
            for other in &self.state.board.snakes {
                if other.id != snake.id
                    && other.status != Status::Dead
                    && snake.lost_head_to_head(other)
                {
                    collision_eliminations.push(snake.id);
                    collide = true;
                    break;
                }
            }
            if collide {
                continue;
            }
        }
        for id in collision_eliminations {
            for snake in &mut self.state.board.snakes {
                if snake.id == id {
                    snake.status = Status::Dead;
                    out.push(snake.id);
                }
            }
        }
        out
    }
    fn unmake_move(&mut self, delta: &Delta) {
        // revive all killed snakes
        for snake in &mut self.state.board.snakes {
            if delta.died.contains(&snake.id) {
                snake.status = Status::Alive;
            }
        }
        // add tails back to snakes
        // and remove all heads

        for snake in &mut self.state.board.snakes {
            for food in &delta.eaten_food {
                if food.0 == snake.id {
                    if !self.state.board.food.contains(&food.2) {
                        self.state.board.food.push(food.2);
                    }
                    snake.health = food.1;
                    snake.body.pop();
                    snake.length -= 1;
                }
            }
            // remove all heads and tails.
            for tail in &delta.tails {
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

    pub fn amnt_dead(&self) -> usize {
        let mut out = 0;
        for snake in &self.state.board.snakes {
            if snake.status == Status::Dead {
                out += 1;
            }
        }
        out
    }
    /// Depth is how far to search
    /// maximizing is whether the function is supposed to be maximizing or minimizing.
    fn minimax(
        &mut self,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
        static_eval: &dyn Fn(&SmallMove, Weights) -> i32,
        you_move: (Direction, u8),
    ) -> (i32, i32, i32, Direction) {
        if self.state.you.status == Status::Dead {
            // println!("{:?}, {}", self.dead, depth);
            // im dead
            return (i32::MIN, alpha, beta, Direction::Up);
        } else if self.state.board.snakes.len() - self.amnt_dead() == 1 {
            // ive won
            // println!("{:?}, {}", self.dead, self.state.you.id);
            return (i32::MAX, alpha, beta, Direction::Up);
        }
        if depth == 0 {
            // let start = Instant::now();
            let x = static_eval(&self.state, self.weights);
            // *count += start.elapsed();
            return (x, alpha, beta, Direction::Up);
        }
        if maximizing {
            let mut value = i32::MIN;
            let mut out = Direction::Up;
            // if self.state.you.get_moves(&self.state.board).len() == 1 {
            //     return (0 , alpha, beta, self.state.you.get_moves(&self.state.board)[0].0);
            // }
            for current_move in self.state.you.get_moves(&self.state.board) {
                // let start = Instant::now();
                // let delta = self.make_move(&vec![(current_move).clone()]);
                // *count += start.elapsed();

                let x = self.minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !maximizing,
                    static_eval,
                    current_move,
                );
                if value < x.0 {
                    out = current_move.0;
                    value = x.0;
                }
                alpha = i32::max(alpha, value);
                if value >= beta {
                    break; // beta cutoff
                }
            }
            (value, alpha, beta, out)
        } else {
            let mut best_move = tinyvec::array_vec!([(Direction, u8); 2]);
            let mut value = i32::MAX;
            for current_move in &self.get_ordered_moves(you_move) {
                // let start = Instant::now();
                let delta = self.make_move(current_move); // make the current move and store the irreversable bits.
                                                          // *count += start.elapsed();
                value = i32::min(
                    value,
                    self.minimax(depth - 1, alpha, beta, !maximizing, static_eval, you_move)
                        .0,
                );
                self.unmake_move(&delta); // unmake the current move
                if beta > value {
                    best_move = *current_move;
                    beta = value;
                }
                if value <= alpha {
                    if best_move != self.move_table.get(self.zobrist) {
                        self.move_table.set(self.zobrist, best_move, depth, value);
                    }
                    self.update_zobrist(current_move); // revert the zobrist hash
                    break;
                }
                self.update_zobrist(current_move); // revert the zobrist hash
            }
            (value, alpha, beta, Direction::Up)
        }
    }
    /// It will return a 2D array of moves for the opposing team.
    fn get_moves(
        &self,
        you_move: (Direction, u8),
    ) -> tinyvec::ArrayVec<[tinyvec::ArrayVec<[(Direction, u8); 2]>; 16]> {
        let mut out = tinyvec::array_vec!([tinyvec::ArrayVec<[(Direction, u8); 4 ]>;2] => tinyvec::array_vec!([(Direction, u8) ; 4] => you_move));
        for x in (&self.state.board.snakes)
            .iter()
            .filter(|x| x.id != self.state.you.id && x.status == Status::Alive)
        {
            out.push(x.get_moves(&self.state.board));
        }
        cartprod::cartesian_product(out)
    }
    fn get_ordered_moves(
        &mut self,
        you_move: (Direction, u8),
    ) -> tinyvec::ArrayVec<[tinyvec::ArrayVec<[(Direction, u8); 2]>; 16]> {
        let mut out = self.get_moves(you_move);
        let best_move = self.move_table.get(self.zobrist);
        // explanation of what follows:
        // Find out if the best move from the table is legal
        //  remove the move from where it is
        //  add it back in at the beginning
        self.tt_hits.0 += 1;
        if let Some(move_pos) = out.iter().position(|x| *x == best_move) {
            out.remove(move_pos);
            out.insert(0, best_move);
            self.tt_hits.1 += 1;
        }
        out
    }
    pub fn iterative_deepen(
        &mut self,
        static_eval: &dyn Fn(&SmallMove, Weights) -> i32,
        time: &Instant,
    ) -> (Direction, i32) {
        let mut depth = 2;
        let mut confidence = 0;
        let mut dir = Direction::Up;
        let max_depth = 130;
        let init_eval = self.minimax(
            1,
            i32::MIN,
            i32::MAX,
            true,
            static_eval,
            (Direction::Up, 40),
        );
        let mut sum = init_eval.0;
        while time.elapsed().as_millis() < 200 && depth <= max_depth {
            let (c, _, _, d) = self.minimax(
                depth,
                i32::MIN,
                i32::MAX,
                true,
                static_eval,
                (Direction::Up, 40),
            );
            sum += c;
            confidence = c;
            dir = d;
            depth += 1;
        }
        println!("avg score {}", sum as f64 / depth as f64);
        println!("Depth searched too: {}", depth);
        (dir, confidence)
    }

    pub fn get_best(
        &mut self,
        static_eval: &dyn Fn(&SmallMove, Weights) -> i32,
        time: &Instant,
    ) -> (Direction, i32) {
        // println!("{:?}", self.state);
        let e = self.clone();
        let moves = self.state.you.get_moves(&self.state.board);
        if moves.is_empty() {
            return (Direction::Up, i32::MIN);
        }
        let out = (moves[0].0, i32::MAX);
        if moves.len() == 1 {
            return out;
        }

        if e.state.board.food != *self.state.board.food {
            println!("{:#?}", e);
            println!("{:#?}", self);
        }
        self.iterative_deepen(static_eval, time)
    }
    pub fn _perft(&mut self, depth: u8, you_move: (Direction, u8), maximizing: bool) -> u32 {
        let mut nodes = 0;
        if self.state.you.status == Status::Dead
            || self.state.board.snakes.len() - self.amnt_dead() == 1
        {
            // println!("e");

            return 1;
        }
        if depth == 0 {
            return 1;
        }

        if maximizing {
            for m in self.state.you.get_moves(&self.state.board) {
                nodes += self._perft(depth, m, !maximizing);
            }
        } else {
            for moves in &self.get_moves(you_move) {
                let delta = self.make_move(moves);
                nodes += self._perft(depth - 1, you_move, !maximizing);
                self.unmake_move(&delta);
            }
        }
        nodes
    }

    // fn get_hash(&self) -> u64 {
    //     let mut out = 0;

    //     out
    // }
}

impl Direction {
    pub fn as_str(&self) -> &str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
    fn bits(&self) -> u8 {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Up
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
