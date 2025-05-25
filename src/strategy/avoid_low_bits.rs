use lazy_static::lazy_static;
use std::collections::HashMap;
use super::{BitFlipStrategy, common::RAW_COMBINATIONS};
use crate::board::Board;

// based on the analysis of combination and the most occurring numbers is best to avoid 1->2->3->4->5 (quite obvious I know)
// superior rn with avg score of 179.15 per game
lazy_static! {
    // Strategy-specific combinations sorted by minimum lever (descending), then max lever (descending)
    static ref NO_LOW_BITS_COMBINATIONS: HashMap<u8, Vec<Vec<u8>>> = {
        let mut m = HashMap::new();
        for sum in 2..=12 {
            if let Some(raw) = (&*RAW_COMBINATIONS).get(&sum) {
                let mut sorted = raw.clone();
                sorted.sort_unstable_by(|a, b| {
                    let min_a = a.iter().min().unwrap();
                    let min_b = b.iter().min().unwrap();
                    min_b.cmp(min_a)
                        .then_with(|| {
                            let max_a = a.iter().max().unwrap();
                            let max_b = b.iter().max().unwrap();
                            max_b.cmp(max_a)
                        })
                });
                m.insert(sum, sorted);
            }
        }
        m
    };

    // Precomputed masks in the same order as NO_LOW_BITS_COMBINATIONS
    static ref NO_LOW_BITS_MASKS: HashMap<u8, Vec<u16>> = {
        (&*NO_LOW_BITS_COMBINATIONS)
            .iter()
            .map(|(&sum, combos)| {
                let masks = combos
                    .iter()
                    .map(|combo| combo_to_mask(combo))
                    .collect();
                (sum, masks)
            })
            .collect()
    };
}

fn combo_to_mask(combo: &[u8]) -> u16 {
    combo.iter().fold(0, |acc, &lever| acc | (1 << (lever - 1)))
}

#[derive(Clone)]
pub struct AvoidLowBitsStrategy;

impl BitFlipStrategy for AvoidLowBitsStrategy {
    fn choose_flip_mask(&self, board: &Board, sum: u8) -> Option<u16> {
        (&*NO_LOW_BITS_MASKS).get(&sum).and_then(|masks| {
            masks.iter()
                .find(|&&mask| (board.0 & mask) == mask)
                .copied()
        })
    }

    fn name(&self) -> &'static str { "Avoid Low Bits" }
    fn description(&self) -> &'static str {
        "Selects valid combination with highest minimum lever, then highest maximum lever"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting_order() {
        let combos = (&*NO_LOW_BITS_COMBINATIONS).get(&12).unwrap();
        // Verify first element has highest minimum lever
        assert_eq!(combos[0], vec![12]);  // min=12
        // Next should be combinations with min=11 (but none exist for sum=12)
        // Then combinations with min=10 (10+2)
        assert!(combos.contains(&vec![10, 2]));
        // Check ordering within same minimum
        let min_5_combos: Vec<_> = combos.iter()
            .filter(|c| c.iter().min() == Some(&5))
            .collect();
        assert!(min_5_combos.windows(2).all(|w| {
            w[0].iter().max() >= w[1].iter().max()
        }));
    }

    #[test]
    fn test_choose_highest_min() {
        let strategy = AvoidLowBitsStrategy;
        let board = Board::new(); // All levers up

        // For sum=7, first combo should be [7] (mask=0b1000000)
        assert_eq!(strategy.choose_flip_mask(&board, 7), Some(1 << 6));

        // For sum=8, first combo is [8] (mask=0b10000000)
        assert_eq!(strategy.choose_flip_mask(&board, 8), Some(1 << 7));
    }

    #[test]
    fn test_avoids_low_bits() {
        let strategy = AvoidLowBitsStrategy;
        let mut board = Board::new();

        // Take lever 6 down (bit 5)
        board.0 &= !(1 << 5);

        // For sum=12:
        // - First choice [12] is invalid (lever 12 is up)
        // - Next would be [11, 1] but min=1 is low
        // - Then [10, 2] (min=2) but lever 2 is up
        // - Actual first valid combo depends on sorted order
        let mask = strategy.choose_flip_mask(&board, 12).unwrap();
        let combo = NO_LOW_BITS_COMBINATIONS[&12].iter()
            .find(|c| combo_to_mask(c) == mask)
            .unwrap();
        assert!(*combo.iter().min().unwrap() >= 2);
    }
}