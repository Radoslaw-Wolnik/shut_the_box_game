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
        if let Some(flip) = perform_turn(board, throw_dice, strategy) {
            board = board ^ flip;
        } else {
            break board.sum_unflipped();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DiceMode, PlayerStrategy};
    use crate::strategy::dummy::DummyHighestStrategy;

    // Fixed dice throw for testing
    fn fixed_throw() -> u8 {
        7 // Always return 7 for predictable tests
    }

    #[test]
    fn test_perform_round() {
        let strategy = PlayerStrategy::Dummy(DummyHighestStrategy);

        // Will try to flip 7 first (success), then next turn try 7 again (fail)
        let score = perform_round(fixed_throw, &strategy);

        // Only lever 7 flipped, sum of remaining 11 levers: 78 (sum 1-12 = 78) - 7 = 71
        assert_eq!(score, 71);
    }
}