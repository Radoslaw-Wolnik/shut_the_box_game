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
) -> TurnResult {
    let sum = throw_dice();
    match strategy.choose_levers_to_flip(&board, sum) {
        Some(levers) => {
            let mut new_board = board;
            new_board.flip_down_multiple(&levers);
            TurnResult { new_board, success: true }
        },
        None => TurnResult { new_board: board, success: false }
    }
}