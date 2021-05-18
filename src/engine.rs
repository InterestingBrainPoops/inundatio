use std::str::FromStr;

use crate::types::*;
// gets the best move given a board state.
pub fn getMove( sent_move: &Move) -> String {
    let out = "up";
    return String::from_str(out).expect("something went horribly wrong.");
}

// Static eval of the board state.
fn eval( board : &Move) -> i32 {
    
    3
}
// makes the following move on the board given.
// Only applies the move to YOU.
// Doesn't clone the board state.
fn make_move(board : &mut Move, move_to_make : &Coordinate){
    match board.you.body.get(0) {
        Some(coord)=> {board.you.body.insert(0, *coord);},
        None => {panic!("something whent wrong");}
    }
    
    board.you.head += *move_to_make;
    board.you.body.pop();
    // find the right snake
    
    for x in &mut board.board.snakes {
        if x.id.eq(&board.you.id) {
            match x.body.get(0) {
                Some(coord)=> {x.body.insert(0, *coord);},
                None => {panic!("something whent wrong");}
            }
            x.head += *move_to_make;
            x.body.pop();
            break;
        }
    }
}