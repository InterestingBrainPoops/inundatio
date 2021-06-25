use std::i32::MIN;

use crate::types::*;
// gets the best move given a board state.

// higher is better
/// Static eval of the board state.
/// returns (reachable food) + (reachable squares) - (distance to target)
pub fn eval(board: &Move, dead: usize) -> i32 {
    {
        // check if dead
        // if dead, then return i32 min
        // then look at the smallest path to the closest food, if there is no path to any food, then return max negative
        // smallest path to the head of the nearest snake that is smaller than it.
        // make an array of all snakes
        // remove all snakes larger than myself, and are myself
        // find the one that has the closest head to me (A*)
        // if no snake is reachable, then add 0 to the score
        // otherwise subtract the length of the path from score.
        // calculate food ownership
        // iterate through each food piece
        // see if I am the closest snake
        // add the number of food i am closest to * 2 to score.
        // calculate area ownership
        // iterate through each empty square
        // see if I am closest to that square
        // add the number of squares i am closest to * 1 to score.
    }
    let mut reachable_squares = vec![];
    flood_fill(board, &board.you.head, &mut reachable_squares);
    reachable_squares.len() as i32
        + (board.you.length * 4 )as i32 - ((board.board.snakes.len()  - dead) * 5) as i32
}
// makes the following move on the board given.
// Only applies the move to YOU.
// Doesn't clone the board state.
// should probably be an implementation.
// also removes all the tails for ALL snakes, not just you.
fn make_move(board: &mut Move, move_to_make: &Coordinate) {
    board.you.head = *move_to_make; // sets the head of the snake
    board.you.body.insert(0, board.you.head); // adds the head to the beginning of the snake
    board.you.body.pop(); // removes the tail
                          // find the right snake
    for x in &mut board.board.snakes {
        if x.id.eq(&board.you.id) {
            x.head = *move_to_make;
            x.body.insert(0, x.head);
            x.body.pop();
        } else {
            x.body.pop();
        }
    }
}
/// returns the manhattan distance between the 2 points.
fn manhattan(pos1: &Coordinate, pos2: &Coordinate) -> i32 {
    ((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()) as i32
}
/// returns whether or not you are dead.
fn lost(board: &Move) -> bool {
    if board.you.head.x < 0
        || board.you.head.x >= board.board.width
        || board.you.head.y < 0
        || board.you.head.y >= board.board.height
    {
        return true; // out of bounds
    }
    for x in &board.board.snakes {
        if manhattan(&board.you.head, &x.head) == 1
            && x.length >= board.you.length
            && board.you.id != x.id
        {
            return true;
        }
        for pos in &x.body[1..] {
            if board.you.head == *pos {
                // collision with a snakes body part
                return true;
            }
        }
    }

    false
}

// the output of this will NEVER be negative.
/// 4 side recursive flood fill implementation
/// Also fills in the counted array, which is what its "output" is.
fn flood_fill(board: &Move, seed: &Coordinate, counted: &mut Vec<Coordinate>) {
    if counted.iter().any(|&i| i == *seed) {
        return;
    }
    if seed.x < 0 || seed.x >= board.board.width || seed.y < 0 || seed.y >= board.board.height {
        // println!("happened, out of bounds");
        return; // out of bounds
    }
    for x in &board.board.snakes {
        for pos in &x.body[..] {
            if *seed == *pos && *seed != board.you.head {
                // not in bounds
                return;
            }
        }
    }
    counted.push(*seed);
    flood_fill(board, &(*seed + Coordinate::new(0, 1)), counted);
    flood_fill(board, &(*seed + Coordinate::new(0, -1)), counted);
    flood_fill(board, &(*seed + Coordinate::new(-1, 0)), counted);
    flood_fill(board, &(*seed + Coordinate::new(1, 0)), counted);
}
