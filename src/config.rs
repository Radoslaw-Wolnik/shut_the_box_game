// use std::env;
use crate::fire_spread::{MooreNeighborhood, VonNeumannNeighborhood};

// Burn pattern options
#[derive(Debug)]
pub enum BurnPattern {
    Moore(MooreNeighborhood),      // 8-directional
    VonNeumann(VonNeumannNeighborhood), // 4-directional
}


// Configuration structure
pub struct Config {
    pub size: usize,
    pub density: f64,
    pub simulations: usize,
    pub burn_pattern: BurnPattern,
    pub display_grid: bool,
    pub frame_delay_ms: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        let mut config = Config {
            size: 20,
            density: 0.6,
            simulations: 1,
            burn_pattern: BurnPattern::Moore(MooreNeighborhood),
            display_grid: true,
            frame_delay_ms: 50,
        };

        let mut args_iter = args.iter().skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-s" | "--size" => {
                    config.size = parse_arg(&mut args_iter, "size")?;
                }
                "-d" | "--density" => {
                    config.density = parse_arg(&mut args_iter, "density")?;
                    if !(0.0..=1.0).contains(&config.density) {
                        return Err("Density must be between 0.0 and 1.0".into());
                    }
                }
                "-c" | "--simulations" => {
                    config.simulations = parse_arg(&mut args_iter, "simulations")?;
                }
                "-b" | "--burn-pattern" => {
                    let pattern = parse_arg::<String>(&mut args_iter, "burn-pattern")?;
                    config.burn_pattern = match pattern.to_lowercase().as_str() {
                        "moore" => BurnPattern::Moore(MooreNeighborhood),
                        "vonneumann" => BurnPattern::VonNeumann(VonNeumannNeighborhood),
                        _ => return Err("Invalid burn pattern. Use 'moore' or 'vonneumann'".into()),
                    };
                }
                "-g" | "--display-grid" => {
                    config.display_grid = true;
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


#[cfg(test)]
mod tests {
    use super::*;

    fn mock_args(args: &[&str]) -> Vec<String> {
        std::iter::once("program_name".to_string())
            .chain(args.iter().map(|s| s.to_string()))
            .collect()
    }

    #[test]
    fn test_default_config() {
        let args = mock_args(&[]);
        let config = Config::new(&args).unwrap();
        assert_eq!(config.size, 50);
        assert_eq!(config.density, 0.6);
        assert_eq!(config.simulations, 100);
        assert!(matches!(config.burn_pattern, BurnPattern::Moore(MooreNeighborhood)));
        assert!(!config.display_grid);
    }

    #[test]
    fn test_full_config() {
        let args = mock_args(&[
            "-s", "100",
            "-d", "0.7",
            "-c", "500",
            "-b", "vonneumann",
            "-g"
        ]);
        let config = Config::new(&args).unwrap();

        assert_eq!(config.size, 100);
        assert!((config.density - 0.7).abs() < f64::EPSILON);
        assert_eq!(config.simulations, 500);
        assert!(matches!(config.burn_pattern, BurnPattern::VonNeumann(VonNeumannNeighborhood)));
        assert!(config.display_grid);
    }

    #[test]
    fn test_invalid_density() {
        let args = mock_args(&["-d", "1.5"]);
        let result = Config::new(&args);
        assert!(result.is_err());
    }
}