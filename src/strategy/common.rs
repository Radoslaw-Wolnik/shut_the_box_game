use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // Raw combinations (unsorted)
    pub static ref RAW_COMBINATIONS: HashMap<u8, Vec<Vec<u8>>> = {
        let mut m = HashMap::new();
        for sum in 2..=12 {
            m.insert(sum, generate_combinations(sum));
        }
        m
    };

    // Raw masks derived directly from RAW_COMBINATIONS (unsorted)
    pub static ref RAW_MASKS: HashMap<u8, Vec<u16>> = {
        (&*RAW_COMBINATIONS)
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

fn generate_combinations(target: u8) -> Vec<Vec<u8>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    generate_combinations_recursive(target, &mut current, target.min(12), &mut results);
    results
}

fn generate_combinations_recursive(
    target: u8,
    current: &mut Vec<u8>,
    start: u8,
    results: &mut Vec<Vec<u8>>,
) {
    if target == 0 {
        results.push(current.clone());
        return;
    }
    if start == 0 {
        return;
    }
    let max = start.min(target);
    for i in (1..=max).rev() {
        current.push(i);
        generate_combinations_recursive(target - i, current, i - 1, results);
        current.pop();
    }
}

fn combo_to_mask(combo: &[u8]) -> u16 {
    combo.iter().fold(0, |acc, &lever| acc | (1 << (lever - 1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_combinations_for_7() {
        let combos = (&*RAW_COMBINATIONS).get(&7).unwrap();
        assert!(combos.contains(&vec![7]));
        assert!(combos.contains(&vec![6, 1]));
        assert!(combos.contains(&vec![5, 2]));
        assert!(combos.contains(&vec![4, 3]));
        assert!(combos.contains(&vec![4, 2, 1]));
    }
}