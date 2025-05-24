// File: common/strategies/strategy_interface.rs

pub struct GameState {
    pub board: u16,
    pub current_round_score: u32,
    pub total_score: u32,
    pub opponent_score: u32,
    pub round: usize,
}

pub struct Move {
    pub bits_to_flip: u16,
}

pub trait Strategy {
    fn choose_move(&self, game_state: &GameState) -> Move;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}