pub mod game;
pub mod result;
mod turn;
pub mod recorder;
type ThrowFn = fn() -> u8;

pub use result::*;