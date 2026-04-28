/*
{
  "simulation_id": 42,
  "turns": 17,
  "winner": "player1",
  "final_board": "111100001111",
  "critical_moments": [
    {"turn": 5, "board": "111111110000", "active_player": 1},
    {"turn": 12, "board": "000000001111", "active_player": 2}
  ]
}
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExtendedGameResult {
    pub simulation_id: u32,
    pub turns: u16,
    pub winner: GameResult,
    pub final_board: String,
    pub critical_moments: Vec<CriticalMoment>,
}

#[derive(Serialize, Deserialize)]
pub struct CriticalMoment {
    pub turn: u16,
    pub board: String,
    pub active_player: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GameResult {
    Player1Wins(u16),
    Player2Wins(u16),
    Tie(u16),
}
impl ExtendedGameResult {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}