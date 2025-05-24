use crate::base::{GameResult, ThrowFn};
use crate::config::{DiceMode, PlayerStrategy};
use crate::dice_throw::{throw_normal, throw_weighted};
use super::round::perform_round;



pub struct Game<'a> {
    throw_dice: ThrowFn,
    player_a_strategy: &'a PlayerStrategy,
    player_b_strategy: &'a PlayerStrategy,
    // board - idk if create a clear board here or not - then we could just clone it instead of initialising in the round
}

impl<'a> Game<'a> {
    /// Construct a new game, picking the throw function once
    pub fn new(
        mode: &DiceMode,
        player_a_strategy: &'a PlayerStrategy,
        player_b_strategy: &'a PlayerStrategy,
    ) -> Self {
        // map enum → fn pointer
        let throw_dice: ThrowFn = match mode {
            DiceMode::Normal   => throw_normal,
            DiceMode::Weighted => throw_weighted,
        };

        Game {
            throw_dice,
            player_a_strategy,
            player_b_strategy,
        }
    }

    pub fn play_verbose(&self, id: u32) -> GameResult {
        let a: [u16; 5] = std::array::from_fn(|_| perform_round(self.throw_dice, self.player_a_strategy));
        let b: [u16; 5] = std::array::from_fn(|_| perform_round(self.throw_dice, self.player_b_strategy));
        GameResult { simulation_id: id, player_a: a, player_b: b }

    }

    pub fn play(&self) -> (u16, u16) {
        let a = (0..5).map(|_| perform_round(self.throw_dice, self.player_a_strategy)).sum();
        let b = (0..5).map(|_| perform_round(self.throw_dice, self.player_b_strategy)).sum();
        (a, b)
    }
}