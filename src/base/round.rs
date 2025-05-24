use crate::base::ThrowFn;
use super::turn::{perform_turn, TurnResult};
use crate::board::Board;
use crate::config::PlayerStrategy;

pub fn perform_round(
    throw_dice: ThrowFn,
    strategy: &PlayerStrategy, // Direct enum reference
) -> u16 {
    let mut board = Board::new();
    loop {
        let TurnResult { new_board, success } =
            perform_turn(board, throw_dice, strategy);
        board = new_board;
        if !success { break board.sum_unflipped() }
    }
}