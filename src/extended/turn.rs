use crate::board::Board;
use crate::config::PlayerStrategy;
use crate::strategy::BitFlipStrategy;
use crate::extended::ThrowFn;

/// returns a mask of bits to flip in the board
pub fn perform_turn(
    board: Board,
    throw_dice: ThrowFn,
    strategy: &PlayerStrategy // directly using the enum
) -> Option<u16> {
    let sum = throw_dice();
    strategy.choose_flip_mask(&board, sum)
}
