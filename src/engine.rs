use crate::types::*;
// gets the best move given a board state.

// higher is better
/// Static eval of the board state.
/// returns (reachable food) + (reachable squares) - (distance to target)
pub fn eval(board: &Move, dead: &Vec<String>) -> i32 {
    let mut closest_pos = (&Coordinate::new(100, 100), 3000);
    for food in &board.board.food {
        if closest_pos.1 > manhattan(food, &board.you.head) {
            closest_pos.1 = manhattan(food, &board.you.head);
            closest_pos.0 = food;
        }
    }
    (board.you.length * 4) as i32
        - ((board.board.snakes.len() - dead.len()) * 5) as i32 - closest_pos.1 / ((board.board.width + board.board.height) as i32) * 3
}

/// returns the manhattan distance between the 2 points.
fn manhattan(pos1: &Coordinate, pos2: &Coordinate) -> i32 {
    ((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()) as i32
}

// the output of this will NEVER be negative.
/// 4 side recursive flood fill implementation
/// Also fills in the counted array, which is what its "output" is.
fn flood_fill(board: &Move, seed: &Coordinate, counted: &mut Vec<Coordinate>, dead: &Vec<String>) {
    if counted.iter().any(|&i| i == *seed) {
        return; // already reached
    }
    if seed.x < 0 || seed.x >= board.board.width || seed.y < 0 || seed.y >= board.board.height {
        return; // out of bounds
    }
    for x in &board.board.snakes {
        if dead.contains(&x.id) {
            continue; // doesnt check against dead snakes.
        }
        for pos in &x.body[..] {
            if *seed == *pos && *seed != board.you.head {
                // not in bounds
                return;
            }
        }
    }
    counted.push(*seed);
    flood_fill(board, &(*seed + Coordinate::new(0, 1)), counted, dead);
    flood_fill(board, &(*seed + Coordinate::new(0, -1)), counted, dead);
    flood_fill(board, &(*seed + Coordinate::new(-1, 0)), counted, dead);
    flood_fill(board, &(*seed + Coordinate::new(1, 0)), counted, dead);
}
