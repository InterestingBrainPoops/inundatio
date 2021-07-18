use crate::small::{SmallMove, Status};
use crate::types::*;
// gets the best move given a board state.

// higher is better
/// Static eval of the board state.
/// returns (reachable food) + (reachable squares) - (distance to target)
pub fn eval(board: &SmallMove) -> i32 {
    let mut closest_pos = (&Coordinate::new(100, 100), 100);
    for food in &board.board.food {
        if closest_pos.1 > manhattan(food, &board.you.head) {
            closest_pos.1 = manhattan(food, &board.you.head);
            closest_pos.0 = food;
        }
    }
    if closest_pos.1 == 100 {
        closest_pos.1 = 0;
    }
    (board.you.length * 1500) as i32
        - ((board.board.snakes.len() - amnt_dead(board)) * 5) as i32
        - closest_pos.1 * 30 + (flood_fill(board)*30) as i32
}

fn amnt_dead(board: &SmallMove) -> usize {
    let mut out = 0;
    for snake in &board.board.snakes {
        if snake.status == Status::Dead {
            out += 1;
        }
    }
    out
}
/// returns the manhattan distance between the 2 points.
fn manhattan(pos1: &Coordinate, pos2: &Coordinate) -> i32 {
    ((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()) as i32
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filled {
    Empty,
    Full
}
// the output of this will NEVER be negative.
/// 4 side recursive flood fill implementation
/// Also fills in the counted array, which is what its "output" is.
fn flood_fill(board: &SmallMove) -> u32 {
    let mut all = vec![vec![Filled::Empty;board.board.width as usize];board.board.height as usize];
    for x in &board.board.snakes {
        for y  in &x.body {
            all[y.x as usize][y.y as usize] = Filled::Empty;
        }
    }
    let mut x1;
    let mut span_above;
    let mut span_below;
    let mut out = 0;
    let mut stack:Vec<Coordinate> = vec![];
    stack.push(board.you.head);
    while stack.len() > 0 {
        let thing = stack.pop().expect("");
        x1 = thing.x;
        while x1 >= 0 && all[thing.x as usize][thing.y as usize] != Filled::Full {x1 -= 1;}
        x1 += 1;
        span_above = false; 
        span_below = false;
        while x1 < thing.x && all[thing.x as usize][thing.y as usize] != Filled::Full {
            all[thing.x as usize][thing.y as usize] = Filled::Full;
            out += 1;
            if !span_above && thing.y > 0 && all[thing.x as usize][thing.y as usize] == Filled::Empty {
                stack.push(Coordinate::new(x1 , thing.y - 1));
                span_above = true;
            }else if span_above && thing.y > 0 && all[thing.x as usize][thing.y as usize] == Filled::Full {
                span_above = false;
            }

            if !span_below && thing.y < board.board.height - 1 && all[thing.x as usize][thing.y as usize] == Filled::Empty {
                stack.push(Coordinate::new(x1, thing.y + 1));
                span_below = true;
            } 
            else if span_below && thing.y < board.board.height - 1 && all[thing.x as usize][thing.y as usize] != Filled::Full {
                span_below = false;
            }
            x1 += 1;
        }
    }
    out

}
