use crate::board::Board;
use crate::strategy::BitFlipStrategy;
use crate::extended::{CriticalMoment, ThrowFn};

pub fn process_player_a_turn(
    board: Board,
    strategy: &dyn BitFlipStrategy,
    throw_dice: ThrowFn,
) -> (Board, Option<CriticalMoment>) {
    let sum = throw_dice();
    let levers = strategy.choose_levers_to_flip(&board, sum);
    let mut new_board = board;
    let critical = levers.as_ref().map(|levers| {
        new_board.flip_down_multiple(levers);
        CriticalMoment {
            turn: 0,  // To be filled by caller
            board: new_board.to_bin_string(),
            active_player: 1,
        }
    });
    (new_board, critical)
}

pub fn process_player_b_turn(
    board: Board,
    strategy: &dyn BitFlipStrategy,
    throw_dice: ThrowFn,
) -> (Board, Option<CriticalMoment>) {
    let sum = throw_dice();
    let levers = strategy.choose_levers_to_flip(&board, sum);
    let mut temp_board = board;
    let critical = levers.as_ref().map(|levers| {
        temp_board.flip_down_multiple(levers);
        let new_board = !temp_board;
        CriticalMoment {
            turn: 0,  // To be filled by caller
            board: new_board.to_bin_string(),
            active_player: 2,
        }
    });
    (if levers.is_some() { !temp_board } else { board }, critical)
}