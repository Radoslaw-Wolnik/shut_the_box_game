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

    pub fn play_verbose(&mut self, simulation_id: u32) -> ExtendedGameResult {
        let mut turns = 0;
        let mut board = Board::new();
        let mut critical_moments = Vec::new();

        loop {
            turns += 1;
            let mut round_ended = false; // unused

            // Process Player A
            let (new_board, critical) = process_player_a_turn(
                board,
                self.player_a_strategy,
                self.throw_dice,
            );
            board = new_board;
            self.process_critical_moment(&mut critical_moments, critical, turns);

            if let Some((result, final_board)) = self.check_verbose_end(turns, &board) {
                return ExtendedGameResult {
                    simulation_id,
                    turns,
                    winner: result,
                    final_board,
                    critical_moments,
                };
            }

            // Process Player B
            let (new_board, critical) = process_player_b_turn(
                board,
                self.player_b_strategy,
                self.throw_dice,
            );
            board = new_board;
            self.process_critical_moment(&mut critical_moments, critical, turns);

            if let Some((result, final_board)) = self.check_verbose_end(turns, &board) {
                return ExtendedGameResult {
                    simulation_id,
                    turns,
                    winner: result,
                    final_board,
                    critical_moments,
                };
            }

            if turns >= 100 {
                return ExtendedGameResult {
                    simulation_id,
                    turns,
                    winner: GameResult::Tie(turns),
                    final_board: board.to_bin_string(),
                    critical_moments,
                };
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

    fn check_verbose_end(&self, turns: u8, board: &Board) -> Option<(GameResult, String)> {
        board.all_same().map(|all_down| {
            let result = if all_down {
                GameResult::Player1Wins(turns)
            } else {
                GameResult::Player2Wins(turns)
            };
            (result, board.to_bin_string())
        })
    }

    fn process_critical_moment(
        &self,
        moments: &mut Vec<CriticalMoment>,
        critical: Option<CriticalMoment>,
        turn: u8,
    ) {
        if let Some(mut cm) = critical {
            cm.turn = turn;
            moments.push(cm);
        }
    }
}