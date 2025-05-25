use std::ops::{BitXor, Not};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board(pub(crate) u16);

const VALID_MASK: u16 = 0b0000_1111_1111_1111; // only lower 12 bits

impl Board {
    pub fn new() -> Self {
        Board(0b1111_1111_1111)
    }

    pub fn is_up(&self, lever: u8) -> bool {
        assert!((1..=12).contains(&lever));
        (self.0 & (1 << (lever - 1))) != 0
    }

    pub fn flip_levers(&mut self, levers: &[u8]) {
        let mask = levers.iter()
            .map(|&lever| 1 << (lever - 1))
            .fold(0, |acc, m| acc | m);
        self.0 ^= mask;
    }


    pub fn sum_unflipped(&self) -> u16 {
        (1..=12)
            .filter(|&lever| self.is_up(lever))
            .sum::<u8>() as u16
    }

    pub fn all_same(&self) -> Option<bool> {
        let bits = self.0;
        if bits == 0b0000_0000_0000 {
            Some(true)  // All down
        } else if bits == 0b1111_1111_1111 {
            Some(false) // All up
        } else {
            None
        }
    }

    // Store board state as hexadecimal
    pub fn to_hex(&self) -> String {
        format!("{:03x}", self.0)
    }

    pub fn to_bin_string(&self) -> String {
        format!("{:012b}", self.0)
    }
}

impl Not for Board {
    type Output = Board;

    fn not(self) -> Board {
        // Flip all 12 bits and mask to keep only the lower 12 bits
        Board(!self.0 & 0b0000_1111_1111_1111)
    }
}

// if i have amsk insted of the [u8] - list of positiosn i can just xor
// You're passing a &[u8], i.e. a list of lever positions, not a precomputed u16 bitmask. So:
// [u8] is not a bitmask; it’s a list like [1, 4, 5]. You must turn that into a u16 before doing any XOR:

impl BitXor<Board> for Board {
    type Output = Board;

    fn bitxor(self, rhs: Board) -> Board {
        Board((self.0 ^ rhs.0) & VALID_MASK)
    }
}
impl BitXor<u16> for Board {
    type Output = Board;

    fn bitxor(self, rhs: u16) -> Board {
        // mask off any stray bits above bit 11,
        // then flip the lower‐12 bits:
        Board((self.0 ^ (rhs & VALID_MASK)) & VALID_MASK)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board_has_all_levers_up() {
        let board = Board::new();
        assert_eq!(board.0, 0b1111_1111_1111);
        assert!(board.all_same() == Some(false));
    }

    #[test]
    fn is_up_checks_correct_bits() {
        let board = Board(0b1010_1010_1010);
        assert!(board.is_up(2));
        assert!(!board.is_up(1));
        assert!(board.is_up(4));
        assert!(!board.is_up(3));
    }

    #[test]
    #[should_panic(expected = "assertion failed: (1..=12).contains(&lever)")]
    fn is_up_panics_on_invalid_lever() {
        let board = Board::new();
        let _ = board.is_up(13);
    }

    #[test]
    fn flip_levers_toggles_correct_bits() {
        let mut board = Board::new();
        // Flip levers 3 and 5
        board.flip_levers(&[3, 5]);
        assert_eq!(board.0, 0b1111_1110_1011);
        // Flip again to restore
        board.flip_levers(&[3, 5]);
        assert_eq!(board.0, 0b1111_1111_1111);
    }

    #[test]
    fn sum_unflipped_calculates_correctly() {
        let mut board = Board::new();
        assert_eq!(board.sum_unflipped(), 78); // 1+2+...+12 = 78

        board.flip_levers(&[12]);
        assert_eq!(board.sum_unflipped(), 78 - 12);

        board.flip_levers(&[1, 2, 3]);
        assert_eq!(board.sum_unflipped(), 78 - 12 - 1 - 2 - 3);
    }

    #[test]
    fn all_same_detection() {
        let full = Board::new();
        assert_eq!(full.all_same(), Some(false));

        let empty = Board(0);
        assert_eq!(empty.all_same(), Some(true));

        let mixed = Board(0b0000_0000_0001);
        assert_eq!(mixed.all_same(), None);
    }

    #[test]
    fn bitwise_operations() {
        // Test NOT
        let full = Board::new();
        assert_eq!(!full, Board(0));
        assert_eq!(!!full, full);

        // Test XOR with Board
        let a = Board(0b1111_0000_1111);
        let b = Board(0b1010_1010_1010);
        assert_eq!(a ^ b, Board(0b0101_1010_0101));

        // Test XOR with u16
        let board = Board::new();
        assert_eq!(board ^ 0b0000_0000_0001, Board(0b1111_1111_1110));
    }

    #[test]
    fn formatting() {
        let board = Board::new();
        assert_eq!(board.to_hex(), "fff");
        assert_eq!(board.to_bin_string(), "111111111111");

        let custom = Board(0b1010_0101_1100);
        assert_eq!(custom.to_hex(), "a5c");
        assert_eq!(custom.to_bin_string(), "101001011100");
    }

    #[test]
    fn mask_handling() {
        // Test that higher bits are masked off
        let board = Board::new() ^ 0b1111_0000_0000_0000;
        assert_eq!(board.0, 0b0000_1111_1111_1111);
    }
}