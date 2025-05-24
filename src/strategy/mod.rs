pub mod greedy;
// pub mod basic

use crate::board::Board;

pub trait BitFlipStrategy: Sync { // not sure why Sync but its a multi threading thingy
    fn choose_levers_to_flip(&self, board: &Board, sum: u8) -> Option<Vec<u8>>; // it would be probably smart to instead of doing a vec of bits to flip just return a u16 - mask of levers to flip
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}