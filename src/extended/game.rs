use std::ops::BitXor;
use crate::board::Board;
use crate::config::{DiceMode, PlayerStrategy};
use crate::dice_throw::{throw_normal, throw_weighted};
use crate::extended::turn::perform_turn;
use crate::extended::{CriticalMoment, ExtendedGameResult, GameResult, ThrowFn};

pub struct Game<'a> {
    throw_dice: ThrowFn,
    player_a_strategy: &'a PlayerStrategy,
    player_b_strategy: &'a PlayerStrategy,
}

impl<'a> Game<'a> {
    pub fn new(
        mode: &DiceMode,
        player_a_strategy: &'a PlayerStrategy,
        player_b_strategy: &'a PlayerStrategy,
    ) -> Self {
        let throw_dice: ThrowFn = match mode {
            DiceMode::Normal => throw_normal,
            DiceMode::Weighted => throw_weighted,
        };

        Game {
            throw_dice,
            player_a_strategy,
            player_b_strategy,
        }
    }

    pub fn play(&mut self) -> GameResult {
        let mut turns = 0;
        let mut board = Board::new();

        loop {
            turns += 1;

            // Player A's turn
            if let Some(flip) = perform_turn(board, self.throw_dice, self.player_a_strategy) {
                board = board ^ flip;
                if let Some(game_result) = self.check_game_end(turns, &board) {
                    return game_result;
                }
            }

            // Player B's turn
            if let Some(flip) = perform_turn(!board, self.throw_dice, self.player_b_strategy) {
                board = board ^ flip;
                if let Some(game_result) = self.check_game_end(turns, &board) {
                    return game_result;
                }
            }

            if turns >= 100 {
                return GameResult::Tie(turns);
            }
        }
    }

    // Helper methods
    fn check_game_end(&self, turns: u8, board: &Board) -> Option<GameResult> {
        board.all_same().map(|all_down| {
            if all_down {
                GameResult::Player1Wins(turns)
            } else {
                GameResult::Player2Wins(turns)
            }
        })
    }

}