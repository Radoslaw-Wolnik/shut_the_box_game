use rand::{Rng};

pub fn throw_dice() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6) + rng.gen_range(1..=6)
}
