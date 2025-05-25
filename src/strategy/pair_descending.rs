use super::BitFlipStrategy;
use crate::board::Board;

/// PairDescendingStrategy: for sum, try single, then (sum-1)+1, (sum-2)+2, etc.
#[derive(Clone)]
pub struct PairDescendingStrategy;

impl BitFlipStrategy for PairDescendingStrategy {
    fn choose_flip_mask(&self, board: &Board, sum: u8) -> Option<u16> {
        // Try single lever first
        if sum >= 1 && sum <= 12 && board.is_up(sum) {
            return Some(1 << (sum - 1));
        }
        // Try two-part splits k + j = sum, with k descending from sum-1 to (sum/2).ceil()
        for k in (1..sum).rev() {
            let j = sum - k;
            if j >= 1 && j < k {
                if board.is_up(k) && board.is_up(j) {
                    let mask = (1 << (k - 1)) | (1 << (j - 1));
                    return Some(mask);
                }
            }
        }
        None
    }

    fn name(&self) -> &'static str { "Pair Descending" }
    fn description(&self) -> &'static str {
        "Flip the highest single lever if possible, otherwise flip descending pairs (sum-1 + 1, sum-2 + 2, ...)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn prefers_single_lever() {
        let strategy = PairDescendingStrategy;
        let board = Board::new(); // All levers up

        // Test sum=7 (should pick single lever)
        let mask = strategy.choose_flip_mask(&board, 7);
        assert_eq!(mask, Some(1 << 6)); // Lever 7 is bit 6

        // Test sum=12 (highest possible)
        let mask = strategy.choose_flip_mask(&board, 12);
        assert_eq!(mask, Some(1 << 11));
    }

    #[test]
    fn falls_back_to_pairs_when_single_unavailable() {
        let strategy = PairDescendingStrategy;

        // Create board with lever 7 down
        let mut board = Board::new();
        board.0 ^= 1 << 6;

        // Sum=7 should try pairs in order: (6,1), (5,2), (4,3)
        // Setup: lever 6 up, 1 down → should skip to next pair
        board.0 ^= 1 << 0; // Flip lever 1 down
        board.0 ^= 1 << 5; // Ensure lever 6 is up
        board.0 ^= 1 << 4; // Flip lever 5 down

        let mask = strategy.choose_flip_mask(&board, 7);
        // Should pick (4,3) since 6 is up but 1 is down
        assert_eq!(mask, Some((1 << 3) | (1 << 2)));
    }

    #[test]
    fn pair_order_respected() {
        let strategy = PairDescendingStrategy;
        let mut board = Board::new();

        // Test sum=8
        // Ideal order: (7,1), (6,2), (5,3), (4,4 invalid)
        // Block (7,1) and (6,2)
        board.0 ^= 1 << 7; // Lever 8 down
        board.0 ^= 1 << 6; // Lever 7 down
        board.0 ^= 1 << 5; // Lever 6 down

        let mask = strategy.choose_flip_mask(&board, 8);
        assert_eq!(mask, Some((1 << 4) | (1 << 2))); // (5,3)
    }

    #[test]
    fn handles_edge_cases() {
        let strategy = PairDescendingStrategy;

        // Test sum=2 (only possible as single lever)
        let mut board = Board::new();
        assert_eq!(strategy.choose_flip_mask(&board, 2), Some(1 << 1));

        // Flip lever 2 down
        board.0 ^= 1 << 1;
        assert_eq!(strategy.choose_flip_mask(&board, 2), None);

        // Test sum=3
        // Possible pairs: (2,1)
        let board = Board::new();
        assert_eq!(strategy.choose_flip_mask(&board, 3), Some(1 << 2)); // Prefers single lever 3
    }

    #[test]
    fn returns_none_when_no_valid_combinations() {
        let strategy = PairDescendingStrategy;
        let mut board = Board::new();

        // Create sum=4 scenario where all options are blocked
        board.0 ^= 1 << 3; // Lever 4 down
        board.0 ^= 1 << 2; // Lever 3 down
        board.0 ^= 1 << 1; // Lever 2 down
        board.0 ^= 1 << 0; // Lever 1 down

        assert_eq!(strategy.choose_flip_mask(&board, 4), None);
    }

    #[test]
    fn complex_scenario() {
        let strategy = PairDescendingStrategy;
        let mut board = Board::new();

        // Sum=10 test
        // Preferred order: 10 → (9,1) → (8,2) → (7,3) → (6,4)
        // Block 10, 9, 8, 7
        board.0 ^= 1 << 9; // 10 down
        board.0 ^= 1 << 8; // 9 down
        board.0 ^= 1 << 7; // 8 down
        board.0 ^= 1 << 6; // 7 down

        let mask = strategy.choose_flip_mask(&board, 10);
        // Should pick (6,4)
        assert_eq!(mask, Some((1 << 5) | (1 << 3)));
    }
}
