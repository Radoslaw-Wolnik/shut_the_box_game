use crate::board::Board;
use crate::strategy::BitFlipStrategy;
use crate::strategy::greedy::GreedyStrategy;
// use std::env;

#[derive(Debug)]
pub enum GameMode {
    Normal,
    Extended
}

pub enum PlayerStrategy {
    Greedy(GreedyStrategy),
    // Basic(BasicStrategy),
}


// Implement BitFlipStrategy for the configuration enum
impl BitFlipStrategy for PlayerStrategy {
    fn choose_levers_to_flip(&self, board: &Board, sum: u8) -> Option<Vec<u8>> {
        match self {
            Self::Greedy(s) => s.choose_levers_to_flip(board, sum),
            // Self::Basic(s) => s.choose_levers_to_flip(board, sum),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            PlayerStrategy::Greedy(s) => s.name()
        }
    }

    fn description(&self) -> &'static str {
        match self {
            PlayerStrategy::Greedy(s) => s.description()
        }
    }
}

#[derive(Debug, Clone)]
pub enum DiceMode {
    Normal,
    Weighted,
}

// Configuration structure
pub struct Config {
    pub game_mode: GameMode,
    pub simulations: usize,
    pub output_file: Option<String>, // correct type for Some<String> // the file name - without the extension
    pub strategy_player_a: PlayerStrategy,
    pub strategy_player_b: PlayerStrategy,
    pub dice_mode: DiceMode,
    pub parallelization: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        let mut config = Config {
            game_mode: GameMode::Normal,
            simulations: 100,
            output_file: None,
            strategy_player_a: PlayerStrategy::Greedy(GreedyStrategy),
            strategy_player_b: PlayerStrategy::Greedy(GreedyStrategy),
            dice_mode: DiceMode::Normal,
            parallelization: false,
        };

        let mut args_iter = args.iter().skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-g" | "--game-mode" => {
                    let mode = parse_arg::<String>(&mut args_iter, "game_mode")?;
                    config.game_mode = match mode.to_lowercase().as_str() {
                        "normal" | "n" => GameMode::Normal,
                        "extended" | "e" => GameMode::Extended,
                        _ => return Err("Invalid game mode use 'normal' or 'extended' or 'n' or 'e' ".into()),
                    };
                }
                "-c" | "--simulations" => {
                    config.simulations = parse_arg(&mut args_iter, "simulations")?;
                }
                "-o" | "--output-file" => {
                    config.output_file = Some(parse_arg(&mut args_iter, "output_file")?);
                }
                // not sure but we could merge sa and sb
                "-a" | "--player-a" | "--player-a-strategy" => {
                    let strategy = parse_arg::<String>(&mut args_iter, "strategy_player_a")?;
                    config.strategy_player_a = match strategy.to_lowercase().as_str() {
                        "greedy" | "g" => PlayerStrategy::Greedy(GreedyStrategy),
                        // "basic" | "b" => PlayerStrategy::Basic,
                        _ => return Err("Invalid player strategy use 'greedy' or 'g' or 'basic' or 'b' ".into()),
                    };
                }
                "-b" | "--player-b" | "--player-b-strategy" => {
                    let strategy = parse_arg::<String>(&mut args_iter, "strategy_player_b")?;
                    config.strategy_player_b = match strategy.to_lowercase().as_str() {
                        "greedy" | "g" => PlayerStrategy::Greedy(GreedyStrategy),
                        // "basic" | "b" => PlayerStrategy::Basic,
                        _ => return Err("Invalid player strategy use 'greedy' or 'g' or 'basic' or 'b' ".into()),
                    };
                }
                "-d" | "--dice-mode" => {
                    let mode = parse_arg::<String>(&mut args_iter, "dice_mode")?;
                    config.dice_mode = match mode.to_lowercase().as_str() {
                        "normal" | "n" => DiceMode::Normal,
                        "weighted" | "w" => DiceMode::Weighted,
                        _ => return Err("Invalid dice mode use 'normal' or 'n' or 'weighted' or 'w' ".into()),
                    };
                }
                "-p" | "--parallelization" => { // idk if there is a community standard for saying if sb wants to do paral.. process
                    config.parallelization = true;
                }
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        }

        Ok(config)
    }
}

fn parse_arg<T: std::str::FromStr>(
    args_iter: &mut dyn Iterator<Item = &String>,
    arg_name: &str,
) -> Result<T, String> { // Result<T, &'static str>
    args_iter.next()
        .ok_or_else(|| format!("Missing value for {}", arg_name))?
        .parse()
        .map_err(|_| format!("Invalid value for {}", arg_name))
}

