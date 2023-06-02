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
use crate::game::board::{Direction, State};
use crate::game::small::{SmallMove, Status};

pub const TABLE_SIZE: usize = 2000000; // At 12 bytes per entry, this causes a size of 24 megabytes.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Weights(pub i32, pub i32, pub i32, pub i32);

 pub fn amnt_dead(state : &State) -> usize {
        let mut out = 0;
        for snake in &state.state.board.snakes {
            if snake.status == Status::Dead {
                out += 1;
            }
        }
        out
    }
    /// Depth is how far to search
    /// maximizing is whether the function is supposed to be maximizing or minimizing.
    fn minimax(
        state : &mut State,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
        static_eval: &dyn Fn(&SmallMove, Weights) -> i32,
        you_move: (Direction, u8),
    ) -> (i32, i32, i32, Direction) {
        if state.state.you.status == Status::Dead {
            // println!("{:?}, {}", self.dead, depth);
            // im dead
            return (i32::MIN, alpha, beta, Direction::Up);
        } else if state.state.board.snakes.len() - state.amnt_dead() == 1 {
            // ive won
            // println!("{:?}, {}", self.dead, self.state.you.id);
            return (i32::MAX, alpha, beta, Direction::Up);
        }
        if depth == 0 {
            // let start = Instant::now();
            let x = static_eval(&state.state, state.weights);
            // *count += start.elapsed();
            return (x, alpha, beta, Direction::Up);
        }
        if maximizing {
            let mut value = i32::MIN;
            let mut out = Direction::Up;
            // if self.state.you.get_moves(&self.state.board).len() == 1 {
            //     return (0 , alpha, beta, self.state.you.get_moves(&self.state.board)[0].0);
            // }
            for current_move in state.state.you.get_moves(&state.state.board) {
                // let start = Instant::now();
                // let delta = self.make_move(&vec![(current_move).clone()]);
                // *count += start.elapsed();

                let x = state.minimax(
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
            for current_move in &state.get_ordered_moves(you_move) {
                // let start = Instant::now();
                let delta = state.make_move(current_move); // make the current move and store the irreversable bits.
                                                          // *count += start.elapsed();
                value = i32::min(
                    value,
                    state.minimax(depth - 1, alpha, beta, !maximizing, static_eval, you_move)
                        .0,
                );
                state.unmake_move(&delta); // unmake the current move
                if beta > value {
                    best_move = *current_move;
                    beta = value;
                }
                if value <= alpha {
                    if best_move != state.move_table.get(state.zobrist) {
                        state.move_table.set(state.zobrist, best_move, depth, value);
                    }
                    state.update_zobrist(current_move); // revert the zobrist hash
                    break;
                }
                state.update_zobrist(current_move); // revert the zobrist hash
            }
            (value, alpha, beta, Direction::Up)
        }
    }
    /// It will return a 2D array of moves for the opposing team.
    fn get_moves(
        state : &State,
        you_move: (Direction, u8),
    ) -> tinyvec::ArrayVec<[tinyvec::ArrayVec<[(Direction, u8); 2]>; 16]> {
        let mut out = tinyvec::array_vec!([tinyvec::ArrayVec<[(Direction, u8); 4 ]>;2] => tinyvec::array_vec!([(Direction, u8) ; 4] => you_move));
        for x in (&state.state.board.snakes)
            .iter()
            .filter(|x| x.id != state.state.you.id && x.status == Status::Alive)
        {
            out.push(x.get_moves(&state.state.board));
        }
        cartprod::cartesian_product(out)
    }
    fn get_ordered_moves(
        state : &mut State,
        you_move: (Direction, u8),
    ) -> tinyvec::ArrayVec<[tinyvec::ArrayVec<[(Direction, u8); 2]>; 16]> {
        let mut out = state.get_moves(you_move);
        let best_move = state.move_table.get(state.zobrist);
        // explanation of what follows:
        // Find out if the best move from the table is legal
        //  remove the move from where it is
        //  add it back in at the beginning
        state.tt_hits.0 += 1;
        if let Some(move_pos) = out.iter().position(|x| *x == best_move) {
            out.remove(move_pos);
            out.insert(0, best_move);
            state.tt_hits.1 += 1;
        }
        out
    }
    pub fn iterative_deepen(state : &mut State,
        static_eval: &dyn Fn(&SmallMove, Weights) -> i32,
        time: &Instant,
    ) -> (Direction, i32) {
        let mut depth = 2;
        let mut confidence = 0;
        let mut dir = Direction::Up;
        let max_depth = 130;
        let init_eval = state.minimax(
            1,
            i32::MIN,
            i32::MAX,
            true,
            static_eval,
            (Direction::Up, 40),
        );
        let mut sum = init_eval.0;
        while time.elapsed().as_millis() < 200 && depth <= max_depth {
            let (c, _, _, d) = state.minimax(
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
    state: State,
    static_eval: &dyn Fn(&SmallMove, Weights) -> i32,
    time: &Instant,
) -> (Direction, i32) {
    // println!("{:?}", self.state);
    let e = state.clone();
    let moves = state.state.you.get_moves(&state.state.board);
    if moves.is_empty() {
        return (Direction::Up, i32::MIN);
    }
    let out = (moves[0].0, i32::MAX);
    if moves.len() == 1 {
        return out;
    }

    if e.state.board.food != *state.state.board.food {
        println!("{:#?}", e);
        println!("{:#?}", state);
    }
    state.iterative_deepen(static_eval, time)
}

