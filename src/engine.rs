use std::{i32::MIN, str::FromStr};

use crate::types::*;
// gets the best move given a board state.
pub fn get_move( sent_move: &Move) -> (&str, Coordinate, i32) {
	let out = "up"; // default move, useless with the design im using.
	let mut possible_moves = vec![("up", Coordinate::new(0, 1), 0 as i32), ("down", Coordinate::new(0, -1), 0 as i32), ("left", Coordinate::new(-1, 0), 0 as i32), ("right", Coordinate::new(1, 0), 0 as i32)];
	// ^ all the possible moves ( 4 of them), their scores, and what they do.
	for x in &mut possible_moves { // iterate through all possible moves
		let mut board_new = sent_move.clone(); // clone the board.
		make_move(&mut board_new, &(sent_move.you.head + x.1)); // make the move on the board
		x.2 = eval(&board_new); // evaluate the board
	}
	// get the highest score, return the tuple.
	let mut biggest = possible_moves[0];
	for x in possible_moves {
		println!("Move: {}, Score: {}", x.0, x.2);
		if x.2 > biggest.2 {
			biggest = x;
		}
	}
	biggest
}

// higher is better
// Static eval of the board state.
fn eval( board : &Move) -> i32 {
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
	if self::lost(board) {return MIN;} // if i lost, then return the minimum 
	let mut food_scores: Vec<(usize, i32)> = vec![(0,0); board.board.food.len()]; // create the foodscores array, preallocate memory.

	for (index, foodpos) in board.board.food.iter().enumerate(){
		food_scores[index] = (index, 0 - manhattan(&board.you.head, foodpos)); // populate the foodscores array
	}
	food_scores.sort_by(|a, b| a.1.cmp(&b.1)); // sort by distance, least to greatest
	
	food_scores[0].1 // return the distance to the closest food.
}
// makes the following move on the board given.
// Only applies the move to YOU.
// Doesn't clone the board state.
fn make_move(board : &mut Move, move_to_make : &Coordinate){
	board.you.head = *move_to_make; // sets the head of the snake
	board.you.body.insert(0, board.you.head); // adds the head to the beginning of the snake
	board.you.body.pop(); // removes the tail
	// find the right snake
	for x in &mut board.board.snakes {
		if x.id.eq(&board.you.id) {
			x.head = *move_to_make;
			x.body.insert(0, x.head);
			x.body.pop();
			break;
		}
	}
}
/// returns the manhattan distance between the 2 points.
fn manhattan(pos1: &Coordinate, pos2: &Coordinate) -> i32 {
	((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()) as i32
}
/// returns whether or not you are dead.
fn lost(board: &Move) -> bool {
	if board.you.head.x < 0 || board.you.head.x >= board.board.width || board.you.head.y < 0 || board.you.head.y >= board.board.height {
		return true;
	}
	for x in &board.board.snakes {
		for pos in &x.body {
			if board.you.head == *pos {
				return true;
			}
		}
	}

	false
}