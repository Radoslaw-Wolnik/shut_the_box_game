use lazy_static::lazy_static;
use std::collections::HashMap;
use super::BitFlipStrategy;
use super::common::RAW_COMBINATIONS;

use crate::board::Board;


// change to make mask not vec<u8>
// no heap allocations
lazy_static! {
    // Strategy-specific sorted combinations
    static ref GREEDY_COMBINATIONS: HashMap<u8, Vec<Vec<u8>>> = {
        let mut m = HashMap::new();
        for sum in 2..=12 {
            if let Some(raw) = (&*RAW_COMBINATIONS).get(&sum) {
                let mut sorted = raw.clone();
                sorted.sort_by(|a, b| {
                    b.len().cmp(&a.len())
                        .then_with(|| b.iter().sum::<u8>().cmp(&a.iter().sum::<u8>()))
                });
                m.insert(sum, sorted);
            }
        }
        m
    };

    // Masks derived from sorted combinations
    static ref GREEDY_MASKS: HashMap<u8, Vec<u16>> = {
        (&*GREEDY_COMBINATIONS)
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
pub struct GreedyStrategy;

impl BitFlipStrategy for GreedyStrategy {

    fn choose_flip_mask(&self, board: &Board, sum: u8) -> Option<u16> {
        (&*GREEDY_MASKS).get(&sum).and_then(|masks| {
            masks.iter().find(|&&mask| (board.0 & mask) == mask).copied()
        })
    }

    fn name(&self) -> &'static str { "Greedy Strategy" }
    fn description(&self) -> &'static str {
        "Selects highest-value valid combination prioritizing most levers then highest sum"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_flip_mask_full_board() {
        let strat = GreedyStrategy;
        let board = Board::new(); // all 12 up
        // Greedy will pick [6,3,2,1] for sum=12 (longest combo), mask = 39
        let mask = strat.choose_flip_mask(&board, 12).unwrap();
        assert_eq!(mask, 39);
    }

    #[test]
    fn test_choose_flip_mask_partial_board() {
        let strat = GreedyStrategy;
        // take lever 6 down
        let mut board = Board::new();
        board = board ^ (1 << (6 - 1));

        // sum=12: now [6,3,2,1] is impossible, next-longest is [5,4,2,1]
        // mask = (1<<4)|(1<<3)|(1<<2) = 28
        let mask = strat.choose_flip_mask(&board, 12).unwrap();
        assert_eq!(mask, (1<<4)|(1<<3)|(1<<1)|(1<<0));
    }

    #[test]
    fn test_combinations_masks_cover() {
        let combos = (&*GREEDY_COMBINATIONS).get(&7).unwrap();
        let masks  = (&*GREEDY_MASKS).get(&7).unwrap();
        // same length
        assert_eq!(combos.len(), masks.len());
        // every combo’s mask should appear
        let computed_masks: Vec<u16> = combos.iter()
            .map(|combo| {
                combo.iter().fold(0u16, |acc, &lever| acc | (1 << (lever - 1)))
            })
            .collect();
        for m in computed_masks {
            assert!(masks.contains(&m));
        }
    }

}