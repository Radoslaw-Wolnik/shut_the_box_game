use rand::{Rng, distributions::WeightedIndex};
use lazy_static::lazy_static;

// using weighted probability for results of two dice throws instead of two rands
// more overhead as simple rand is well optimised in rust and distribution::WeightedIndex is less
lazy_static! {
    static ref WEIGHTS: WeightedIndex<u8> = {
        // Probabilities for sums 2-12 in 36th increments
        let weights = [1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1];
        WeightedIndex::new(&weights).unwrap()
    };
}
pub fn throw_dice() -> u8 {
    let mut rng = rand::thread_rng();
    // Map sampled index (0-10) to dice sum (2-12)
    (rng.sample(&*WEIGHTS) + 2) as u8
}