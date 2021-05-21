use std::{i32::MIN, str::FromStr};

use crate::types::*;
// gets the best move given a board state.
pub fn get_move( sent_move: &Move) -> (&str, Coordinate, i32) {
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
	let target;
	for (index, foodpos) in board.board.food.iter().enumerate(){
		food_scores[index] = (index, manhattan(&board.you.head, foodpos)); // populate the foodscores array
	}
	food_scores.sort_by(|a, b| a.1.cmp(&b.1)); // sort by distance, least to greatest
	
	// get the smallest snake
	let mut smallest = &board.board.snakes[0];
	for snake in &board.board.snakes { // loop through all snakes
		if smallest.length > snake.length && snake.id.ne(&board.you.id) { // if smallest isn't the smallest, make it the smallest
			smallest = snake;
		}
	}
	if smallest.length < board.you.length && board.board.snakes.len() != 0 {
		target = smallest.head;
		println!("Choosing Snake at distance {}", manhattan(&board.you.head, &target))
	}else{
		target = board.board.food[food_scores[0].0];
		println!("Choosing Food at distance {}", food_scores[0].1);
	}
	let mut count = 0;
	let mut depth = 5;
	flood_fill(board, &board.you.head, &mut count, &mut depth);
	println!("FF SCORE : {}", count);
	0 - manhattan(&board.you.head, &target) + count// return the target value, but negative. thus lower equals higher.
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
			return;
		}
	}
	panic!("Should never be reached (function call to make_move terminated without reaching the break statement)");
}
/// returns the manhattan distance between the 2 points.
fn manhattan(pos1: &Coordinate, pos2: &Coordinate) -> i32 {
	((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()) as i32
}
/// returns whether or not you are dead.
fn lost(board: &Move) -> bool {
	if board.you.head.x < 0 || board.you.head.x >= board.board.width || board.you.head.y < 0 || board.you.head.y >= board.board.height {
		return true; // out of bounds
	}
	for x in &board.board.snakes {
		if manhattan(&board.you.head, &x.head) == 1 && x.length >= board.you.length && board.you.id != x.id {
			return true;
		}
		for pos in &x.body[1..] {
			if board.you.head == *pos{
				// collision with a snakes body part
				return true;
			}
			
		}
	}

	false
}

// the output of this will NEVER be negative. 
/// 4 side recursive flood fill implementation 
/// depth limited to prevent stack overflows
fn flood_fill (board : &Move , seed: &Coordinate, count : &mut i32, depth : &mut i32) {
	if(*depth == 0) {
		return;
	}
	if seed.x < 0 || seed.x >= board.board.width || seed.y < 0 || seed.y >= board.board.height {
		return ; // out of bounds
	}
	for x in &board.board.snakes {
		for pos in &x.body[1..] {
			if *seed == *pos {
				// not in bounds
				return;
			}
		}
	}
	*depth -= 1;
	*count += 1;
	flood_fill(board, &(*seed + Coordinate::new(0,1)), count, depth);
	flood_fill(board, &(*seed + Coordinate::new(0,-1)), count, depth);
	flood_fill(board, &(*seed + Coordinate::new(-1,0)), count, depth);
	flood_fill(board, &(*seed + Coordinate::new(1,0)), count, depth);
}
