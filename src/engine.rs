use std::str::FromStr;

use crate::types::*;
// gets the best move given a board state.
pub fn getMove( sent_move: &Move) -> String {
	let out = "up";
	return String::from_str(out).expect("something went horribly wrong.");
}

// higher is better
// Static eval of the board state.
fn eval( board : &Move) -> i32 {
	let score = 0;
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

	score
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