use std::ops::{Index, IndexMut};
use crate::game::board::Direction;

use crate::types::{Direction, TABLE_SIZE};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoveTable {
    table: Vec<Entry>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    moves: [(Direction, u8); 2],
    score: i32,
    depth: u8,
}

impl Index<u64> for MoveTable {
    type Output = Entry;

    fn index(&self, index: u64) -> &Self::Output {
        return &self.table[(index % TABLE_SIZE as u64) as usize];
    }
}

impl IndexMut<u64> for MoveTable {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        return &mut self.table[(index % TABLE_SIZE as u64) as usize];
    }
}

impl MoveTable {
    pub fn new() -> Self {
        MoveTable {
            table: vec![
                Entry {
                    moves: [(Direction::Up, 2), (Direction::Up, 1)],
                    score: 0,
                    depth: 200,
                };
                TABLE_SIZE
            ],
        }
    }

    /// Get a direction for a given position.
    pub fn get(&self, position: u64) -> tinyvec::ArrayVec<[(Direction, u8); 2]> {
        tinyvec::array_vec!([(Direction, u8);2] => self[position].moves[0], self[position].moves[1])
    }
    /// Set a specific position with a specific move.
    pub fn set(
        &mut self,
        position: u64,
        directions: tinyvec::ArrayVec<[(Direction, u8); 2]>,
        depth: u8,
        score: i32,
    ) {
        // println!("{:?}", directions);
        self[position] = Entry {
            depth,
            moves: [directions[0], directions[1]],
            score,
        };
    }
}
