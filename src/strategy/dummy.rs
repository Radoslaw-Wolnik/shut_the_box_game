use super::BitFlipStrategy;
use crate::board::Board;

/// Dummy strategy: always flip only the single lever equal to the sum.
#[derive(Clone)]
pub struct DummyHighestStrategy;

impl BitFlipStrategy for DummyHighestStrategy {
    fn choose_flip_mask(&self, board: &Board, sum: u8) -> Option<u16> {
        if sum >= 1 && sum <= 12 && board.is_up(sum) {
            Some(1 << (sum - 1))
        } else {
            None
        }
    }
    fn name(&self) -> &'static str { "Dummy Highest" }
    fn description(&self) -> &'static str {
        "Always tries to flip only the single lever equal to the dice sum"
    }
}