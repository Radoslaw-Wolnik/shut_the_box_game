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