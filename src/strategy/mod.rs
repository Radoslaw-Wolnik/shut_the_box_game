pub mod greedy;
// pub mod basic

use crate::board::Board;

pub trait BitFlipStrategy: Sync { // not sure why Sync, but it's a multi threading thingy
    /// Given the current board and dice-sum, return the u16 mask of levers to flip
    fn choose_flip_mask(&self, board: &Board, sum: u8) -> Option<u16>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}