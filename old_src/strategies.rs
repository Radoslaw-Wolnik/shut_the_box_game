
use rand::Rng;
use std::collections::HashSet;
use crate::common::strategy_interface::{Strategy, GameState, Move};

const MAX_LOOK_AHEAD: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum StrategyComponent {
    Random,
    HighestValue,
    HighestProbability,
    BalancedValue,
    Adaptive,
    LookAhead,
    ScoreManagement,
    RiskAverse,
    Aggressive,
    PatternRecognition,
}

// Individual strategy implementations
pub struct RandomStrategy;
pub struct HighestValueStrategy;
pub struct HighestProbabilityStrategy;
pub struct BalancedValueStrategy;
pub struct AdaptiveStrategy;
pub struct LookAheadStrategy;
pub struct ScoreManagementStrategy;
pub struct RiskAverseStrategy;
pub struct AggressiveStrategy;
pub struct PatternRecognitionStrategy;

impl Strategy for RandomStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let mut rng = rand::thread_rng();
        let available_bits: Vec<u8> = (1..=12)
            .filter(|&i| game_state.board & (1 << (i - 1)) != 0)
            .collect();
        
        if available_bits.is_empty() {
            return Move { bits_to_flip: 0 };
        }
        
        let chosen_bit = available_bits[rng.gen_range(0..available_bits.len())];
        Move { bits_to_flip: 1 << (chosen_bit - 1) }
    }

    fn name(&self) -> &'static str { "Random" }
    fn description(&self) -> &'static str { "Chooses moves randomly" }
}

impl Strategy for HighestValueStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        for bit in (1..=12).rev() {
            if game_state.board & (1 << (bit - 1)) != 0 {
                return Move { bits_to_flip: 1 << (bit - 1) };
            }
        }
        Move { bits_to_flip: 0 }
    }

    fn name(&self) -> &'static str { "Highest Value" }
    fn description(&self) -> &'static str { "Always chooses the highest available value" }
}

impl Strategy for HighestProbabilityStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let probabilities = [7, 6, 8, 5, 9, 4, 10, 3, 11, 2, 12];
        for &bit in &probabilities {
            if game_state.board & (1 << (bit - 1)) != 0 {
                return Move { bits_to_flip: 1 << (bit - 1) };
            }
        }
        Move { bits_to_flip: 0 }
    }

    fn name(&self) -> &'static str { "Highest Probability" }
    fn description(&self) -> &'static str { "Chooses based on dice roll probability" }
}

impl Strategy for BalancedValueStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let balanced_order = [7, 8, 6, 9, 5, 10, 4, 11, 3, 12, 2];
        for &bit in &balanced_order {
            if game_state.board & (1 << (bit - 1)) != 0 {
                return Move { bits_to_flip: 1 << (bit - 1) };
            }
        }
        Move { bits_to_flip: 0 }
    }

    fn name(&self) -> &'static str { "Balanced Value" }
    fn description(&self) -> &'static str { "Balances between value and probability" }
}

impl Strategy for AdaptiveStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let bits_on = game_state.board.count_ones();
        if bits_on > 6 {
            HighestValueStrategy.choose_move(game_state)
        } else {
            HighestProbabilityStrategy.choose_move(game_state)
        }
    }

    fn name(&self) -> &'static str { "Adaptive" }
    fn description(&self) -> &'static str { "Adapts strategy based on game state" }
}

impl Strategy for LookAheadStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let mut best_move = Move { bits_to_flip: 0 };
        let mut best_score = i32::MIN;

        for bit in 1..=12 {
            if game_state.board & (1 << (bit - 1)) != 0 {
                let mut new_state = game_state.clone();
                new_state.board &= !(1 << (bit - 1));
                
                let score = self.evaluate_position(&new_state, 3); // Look 3 moves ahead
                if score > best_score {
                    best_score = score;
                    best_move = Move { bits_to_flip: 1 << (bit - 1) };
                }
            }
        }

        best_move
    }

    fn name(&self) -> &'static str { "Look Ahead" }
    fn description(&self) -> &'static str { "Evaluates future positions" }
}

impl LookAheadStrategy {
    fn evaluate_position(&self, state: &GameState, depth: i32) -> i32 {
        if depth == 0 {
            return -(state.board.count_ones() as i32);
        }

        let mut best_score = i32::MIN;
        for bit in 1..=12 {
            if state.board & (1 << (bit - 1)) != 0 {
                let mut new_state = state.clone();
                new_state.board &= !(1 << (bit - 1));
                let score = -self.evaluate_position(&new_state, depth - 1);
                best_score = best_score.max(score);
            }
        }
        best_score
    }
}

impl Strategy for ScoreManagementStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let score_diff = game_state.total_score as i32 - game_state.opponent_score as i32;
        if score_diff > 20 {
            HighestProbabilityStrategy.choose_move(game_state) // Play safe if ahead
        } else if score_diff < -20 {
            HighestValueStrategy.choose_move(game_state) // Take risks if behind
        } else {
            BalancedValueStrategy.choose_move(game_state) // Play balanced if close
        }
    }

    fn name(&self) -> &'static str { "Score Management" }
    fn description(&self) -> &'static str { "Adjusts strategy based on score difference" }
}

impl Strategy for RiskAverseStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let mut best_move = Move { bits_to_flip: 0 };
        let mut min_risk = u32::MAX;

        for bit in 1..=12 {
            if game_state.board & (1 << (bit - 1)) != 0 {
                let risk = bit as u32; // Higher numbers are considered riskier
                if risk < min_risk {
                    min_risk = risk;
                    best_move = Move { bits_to_flip: 1 << (bit - 1) };
                }
            }
        }

        best_move
    }

    fn name(&self) -> &'static str { "Risk Averse" }
    fn description(&self) -> &'static str { "Prefers safer, lower-value moves" }
}

impl Strategy for AggressiveStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let mut best_move = Move { bits_to_flip: 0 };
        let mut max_value = 0;

        for bit in 1..=12 {
            if game_state.board & (1 << (bit - 1)) != 0 {
                let value = bit;
                if value > max_value {
                    max_value = value;
                    best_move = Move { bits_to_flip: 1 << (bit - 1) };
                }
            }
        }

        best_move
    }

    fn name(&self) -> &'static str { "Aggressive" }
    fn description(&self) -> &'static str { "Always goes for the highest value moves" }
}

impl Strategy for PatternRecognitionStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        // This is a simplified pattern recognition.
        // In a real implementation, you might want to use more sophisticated pattern matching algorithms.
        let mut move_scores = vec![(0u16, 0f64); 12];
        
        for bit in 1..=12 {
            if game_state.board & (1 << (bit - 1)) != 0 {
                let score = if game_state.round > 0 && game_state.board & (1 << (bit - 1)) != 0 {
                    5.0 // Favor moves that were available in the previous round
                } else {
                    0.0
                };
                move_scores[bit as usize - 1] = (1 << (bit - 1), score);
            }
        }

        move_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Move { bits_to_flip: move_scores[0].0 }
    }

    fn name(&self) -> &'static str { "Pattern Recognition" }
    fn description(&self) -> &'static str { "Recognizes patterns in game progression" }
}


pub struct CompositeStrategy {
    components: HashSet<StrategyComponent>,
    name: String,
    description: String,
}

impl CompositeStrategy {
    pub fn new(components: &[StrategyComponent], name: &str, description: &str) -> Self {
        CompositeStrategy {
            components: components.iter().cloned().collect(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }

    pub fn contains(&self, component: StrategyComponent) -> bool {
        self.components.contains(&component)
    }
}

impl Strategy for CompositeStrategy {
    fn choose_move(&self, game_state: &GameState) -> Move {
        let possible_moves = generate_possible_moves(game_state.board);
        
        if possible_moves.is_empty() {
            return Move { bits_to_flip: 0 };
        }
        
        let chosen_move = self.choose_composite_move(game_state, &possible_moves);
        Move { bits_to_flip: chosen_move }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl CompositeStrategy {
    fn choose_composite_move(&self, state: &GameState, possible_moves: &[u16]) -> u16 {
        let mut move_scores: Vec<(u16, f64)> = possible_moves.iter().map(|&m| (m, 0.0)).collect();

        for component in &self.components {
            match component {
                StrategyComponent::Random => random_score(&mut move_scores),
                StrategyComponent::HighestValue => highest_value_score(&mut move_scores),
                StrategyComponent::HighestProbability => highest_probability_score(&mut move_scores),
                StrategyComponent::BalancedValue => balanced_value_score(&mut move_scores),
                StrategyComponent::Adaptive => adaptive_score(state, &mut move_scores),
                StrategyComponent::LookAhead => look_ahead_score(state, &mut move_scores),
                StrategyComponent::ScoreManagement => score_management_score(state, &mut move_scores),
                StrategyComponent::RiskAverse => risk_averse_score(&mut move_scores),
                StrategyComponent::Aggressive => aggressive_score(&mut move_scores),
                StrategyComponent::PatternRecognition => pattern_recognition_score(state, &mut move_scores),
            }
        }

        move_scores.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0
    }
}

fn generate_possible_moves(board: u16) -> Vec<u16> {
    let mut moves = Vec::new();
    for i in 1..=12 {
        if board & (1 << (i - 1)) != 0 {
            moves.push(1 << (i - 1));
        }
    }
    moves
}

// Implement scoring functions
fn random_score(move_scores: &mut [(u16, f64)]) {
    let mut rng = rand::thread_rng();
    for (_, score) in move_scores.iter_mut() {
        *score += rng.gen::<f64>();
    }
}

fn highest_value_score(move_scores: &mut [(u16, f64)]) {
    for (m, score) in move_scores.iter_mut() {
        *score += m.trailing_zeros() as f64;
    }
}

fn highest_probability_score(move_scores: &mut [(u16, f64)]) {
    for (m, score) in move_scores.iter_mut() {
        let center = (m.trailing_zeros() as i32 - 7).abs();
        *score += 7.0 - center as f64;
    }
}

fn balanced_value_score(move_scores: &mut [(u16, f64)]) {
    for (m, score) in move_scores.iter_mut() {
        let value = m.trailing_zeros() as f64;
        let probability = 7.0 - (value - 7.0).abs();
        *score += value * probability;
    }
}

fn adaptive_score(state: &GameState, move_scores: &mut [(u16, f64)]) {
    let score_diff = state.total_score as i32 - state.opponent_score as i32;
    for (m, score) in move_scores.iter_mut() {
        if score_diff > 0 {
            // If ahead, prefer safer moves
            *score += highest_probability_score(&mut [(m, 0.0)])[0].1;
        } else {
            // If behind, prefer higher value moves
            *score += highest_value_score(&mut [(m, 0.0)])[0].1;
        }
    }
}

fn look_ahead_score(state: &GameState, move_scores: &mut [(u16, f64)]) {
    for (m, score) in move_scores.iter_mut() {
        *score += evaluate_move(state, *m, MAX_LOOK_AHEAD) as f64;
    }
}

fn score_management_score(state: &GameState, move_scores: &mut [(u16, f64)]) {
    // Implementation remains the same
}

fn risk_averse_score(move_scores: &mut [(u16, f64)]) {
    for (m, score) in move_scores.iter_mut() {
        let risk = m.count_ones() as f64;  // More bits flipped = higher risk
        *score -= risk;
    }
}

fn aggressive_score(move_scores: &mut [(u16, f64)]) {
    for (m, score) in move_scores.iter_mut() {
        let aggression = m.count_ones() as f64;  // More bits flipped = more aggressive
        *score += aggression;
    }
}

fn pattern_recognition_score(state: &GameState, move_scores: &mut [(u16, f64)]) {
    // This is a simplified pattern recognition.
    for (m, score) in move_scores.iter_mut() {
        if state.turn_history.contains(m) {
            *score += 5.0;  // Favor moves that have been successful in the past
        }
    }
}

// Helper functions
fn evaluate_move(state: &GameState, move_: u16, depth: usize) -> i32 {
    // Implementation remains the same
}

pub fn create_strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        Box::new(RandomStrategy),
        Box::new(HighestValueStrategy),
        Box::new(HighestProbabilityStrategy),
        Box::new(BalancedValueStrategy),
        Box::new(AdaptiveStrategy),
        Box::new(LookAheadStrategy),
        Box::new(CompositeStrategy::new(
            &[StrategyComponent::Adaptive, StrategyComponent::ScoreManagement],
            "Adaptive Score Management",
            "Adapts strategy based on game state and manages score"
        )),
        Box::new(CompositeStrategy::new(
            &[StrategyComponent::LookAhead, StrategyComponent::ScoreManagement],
            "Look Ahead Score Management",
            "Looks ahead and manages score"
        )),
        Box::new(CompositeStrategy::new(
            &[StrategyComponent::LookAhead, StrategyComponent::Adaptive, StrategyComponent::ScoreManagement],
            "Advanced Adaptive",
            "Advanced strategy combining look-ahead, adaptation, and score management"
        )),
        Box::new(CompositeStrategy::new(
            &[StrategyComponent::BalancedValue, StrategyComponent::RiskAverse, StrategyComponent::PatternRecognition],
            "Cautious Pattern Learner",
            "Balances value while being risk-averse and recognizing patterns"
        )),
        Box::new(CompositeStrategy::new(
            &[StrategyComponent::HighestValue, StrategyComponent::Aggressive, StrategyComponent::LookAhead],
            "Aggressive Planner",
            "Aggressively plans ahead focusing on highest values"
        )),
    ]
}