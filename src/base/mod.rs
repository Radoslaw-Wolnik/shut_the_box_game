pub mod game;
pub mod result;
mod round;
mod turn;
pub mod recorder;

type ThrowFn = fn() -> u8;

pub use result::*;