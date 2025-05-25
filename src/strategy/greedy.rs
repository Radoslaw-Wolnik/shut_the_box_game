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
    fn test_combinations_for_7() {
        // let combinations = COMBINATIONS.get(&7).unwrap();
        let combinations = (*COMBINATIONS).get(&7).unwrap();
        assert!(combinations.contains(&vec![7]));
        assert!(combinations.contains(&vec![6, 1]));
        assert!(combinations.contains(&vec![5, 2]));
        assert!(combinations.contains(&vec![4, 3]));
        assert!(combinations.contains(&vec![4, 2, 1]));
    }
}