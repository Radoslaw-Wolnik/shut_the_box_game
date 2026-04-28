mod board;
mod dice_throw;
mod strategy;
mod config;
mod base;
mod extended;

use std::env;
use std::path::Path;
use std::time::Instant;
use crate::board::Board;
use crate::config::{Config, GameMode};
use crate::strategy::BitFlipStrategy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::new(&args)
        .map_err(|e| format!("Configuration error: {}", e))?;

    let strategy_a = &config.strategy_player_a;  // Get reference to enum
    let strategy_b = &config.strategy_player_b;

    match config.game_mode {
        GameMode::Normal => {
            let game = base::game::Game::new(&config.dice_mode, strategy_a, strategy_b);

            if config.output_file.is_none(){
                // no recorder, no file output just console write
                let mut stats = base::result::GameStatistics::new();
                let start = Instant::now();

                for i in 0..config.simulations as u32 {
                    let (a, b) = game.play();
                    println!("Sim {} → A: {}, B: {}", i, a, b);
                    stats.update(a, b);
                }
                let elapsed = start.elapsed();
                stats.print(config.simulations as u32);
                println!("Elapsed: {:.2?}", elapsed);

                let max_score = 12 + 11 + 10 + 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1;
                println!("Max score per round: {} and per game {}", max_score, max_score * 5); // avg 269/5 - 53 per round

            } else {
                // record verbose data
                // Determine output path
                let output_path = generate_output_path(config.output_file.unwrap().as_str(), GameMode::Normal);
                // do simulations in batch mode
                let mut recorder = base::recorder::Recorder::new(&*output_path);
                for i in 0..config.simulations as u32 {
                    let result = game.play_verbose(i);
                    recorder.record(&result);
                }
                recorder.flush()?;
            }
        }
        GameMode::Extended => {
            let mut game = extended::game::Game::new(&config.dice_mode, strategy_a, strategy_b);

            if config.output_file.is_none(){
                // not saving data to file
                let start = Instant::now();

                for i in 0..config.simulations as u32 {
                    let result = game.play();
                    println!("{:?}", result);
                }

                let elapsed = start.elapsed();
                println!("Elapsed: {:.2?}", elapsed);
            } else {
                // saving data to file

            }
        }
    }

    Ok(())
}


/// Prepares the output file path according to config and existing files.
/// Returns Some(path) if recording, or None for console-only.
fn generate_output_path(base_name: &str, mode: GameMode) -> String {
    let ext = match mode {
        GameMode::Normal => "csv",
        GameMode::Extended => "txt",
    };
    let mut candidate = format!("{}.{}", base_name, ext);
    if Path::new(&candidate).exists() {
        let mut i = 1;
        loop {
            let name = format!("{}_{}.{}", base_name, i, ext);
            if !Path::new(&name).exists() {
                candidate = name;
                break;
            }
            i += 1;
        }
    }
    candidate
}