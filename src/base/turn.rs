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
    fn test_perform_turn_success() {
        let board = Board::new(); // All levers up
        let strategy = PlayerStrategy::Dummy(DummyHighestStrategy);

        let result = perform_turn(board, fixed_throw, &strategy);
        assert_eq!(result, Some(1 << 6)); // Lever 7 is bit 6
    }

    #[test]
    fn test_perform_turn_failure() {
        let mut board = Board::new();
        board.0 ^= 1 << 6; // Flip lever 7 down
        let strategy = PlayerStrategy::Dummy(DummyHighestStrategy);

        let result = perform_turn(board, fixed_throw, &strategy);
        assert_eq!(result, None);
    }
}