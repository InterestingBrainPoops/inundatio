use crate::types::{Direction, TABLE_SIZE};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoveTable {
    table: Box<[[(Direction, u8); 2]; TABLE_SIZE]>,
}
impl MoveTable {
    pub fn new() -> Self {
        MoveTable {
            table: Box::new([[(Direction::Up, 0), (Direction::Up, 1)]; TABLE_SIZE]),
        }
    }
    /// Get a direction for a given position.
    pub fn get(&self, position: u64) -> tinyvec::ArrayVec<[(Direction, u8); 2]> {
        tinyvec::array_vec!([(Direction, u8);2] => self.table[(position % TABLE_SIZE as u64) as usize][0], self.table[(position % TABLE_SIZE as u64) as usize][1])
    }
    /// Set a specific position with a specific move.
    pub fn set(&mut self, position: u64, dir: tinyvec::ArrayVec<[(Direction, u8); 2]>) {
        self.table[(position % TABLE_SIZE as u64) as usize] = [dir[0], dir[1]];
    }
}
