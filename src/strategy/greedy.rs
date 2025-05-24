use lazy_static::lazy_static;
use std::collections::HashMap;
use super::BitFlipStrategy;
use crate::board::Board;

lazy_static! {
    static ref COMBINATIONS: HashMap<u8, Vec<Vec<u8>>> = {
        let mut map = HashMap::new();
        for sum in 2..=12 {
            let mut combinations = generate_combinations(sum);
            combinations.sort_by(|a, b| {
                b.len().cmp(&a.len()).then_with(||
                    b.iter().sum::<u8>().cmp(&a.iter().sum::<u8>())
                )
            });
            map.insert(sum, combinations);
        }
        map
    };
}

fn generate_combinations(target: u8) -> Vec<Vec<u8>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    generate_combinations_recursive(target, &mut current, target.min(12), &mut results);
    results
}

fn generate_combinations_recursive(target: u8, current: &mut Vec<u8>, start: u8, results: &mut Vec<Vec<u8>>) {
    if target == 0 {
        results.push(current.clone());
        return;
    }
    if start == 0 {
        return;
    }
    let start = start.min(target);
    for i in (1..=start).rev() {
        current.push(i);
        generate_combinations_recursive(target - i, current, i - 1, results);
        current.pop();
    }
}
#[derive(Clone)]
pub struct GreedyStrategy;

impl BitFlipStrategy for GreedyStrategy {
    fn choose_levers_to_flip(&self, board: &Board, sum: u8) -> Option<Vec<u8>> {
        /* The problem you’re running into is that COMBINATIONS—as declared by lazy_static!
         * - is not itself a HashMap<…>, but a wrapper type (Lazy<HashMap<…>>) that deref-coerces to your map.
         * In almost every context Rust will insert the deref for you, but for some reason in your setup it isn’t,
         * so COMBINATIONS.get(…) is trying to call get on the Lazy<…> wrapper, not the inner HashMap.
         */
        // let combinations = COMBINATIONS.get(&sum)?; //
        let combinations = (&*COMBINATIONS).get(&sum)?; // or let combinations = (*COMBINATIONS).get(&sum)?;
        for combination in combinations {
            if combination.iter().all(|&lever| board.is_up(lever)) {
                return Some(combination.clone());
            }
        }
        None
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