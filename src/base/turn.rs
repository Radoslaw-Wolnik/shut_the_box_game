use crate::{board::Board};
use crate::base::ThrowFn;
use crate::config::PlayerStrategy;
use crate::strategy::BitFlipStrategy;

pub struct TurnResult {
    pub new_board: Board,
    pub success: bool,
}

pub fn perform_turn(
    board: Board,
    throw_dice: ThrowFn,
    strategy: &PlayerStrategy // directly using the enum
) -> Option<u16> {
    let sum = throw_dice();
    strategy.choose_flip_mask(&board, sum)
}