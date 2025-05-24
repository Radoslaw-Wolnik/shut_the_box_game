use serde::Serialize;
#[derive(Serialize)]
pub struct GameResult {
    pub simulation_id: u32,
    pub player_a: [u16; 5],
    pub player_b: [u16; 5],
}

impl GameResult {
    pub fn to_csv_line(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            self.simulation_id,
            self.player_a.iter().sum::<u16>(),
            self.player_b.iter().sum::<u16>(),
            self.player_a[0], self.player_a[1], self.player_a[2], self.player_a[3], self.player_a[4],
            self.player_b[0], self.player_b[1], self.player_b[2], self.player_b[3], self.player_b[4]
        )
    }
}

#[derive(Default)]
pub struct GameStatistics {
    total_score: f64,
    min_score: f64,
    max_score: f64,
    wins_a: u32,
    wins_b: u32,
}

impl GameStatistics {
    pub(crate) fn new() -> Self {
        Self {
            min_score: f64::MAX,
            max_score: f64::MIN,
            ..Default::default()
        }
    }

    pub(crate) fn update(&mut self, a: u16, b: u16) {
        self.total_score += (a + b) as f64;
        self.min_score = self.min_score.min(a.min(b) as f64);
        self.max_score = self.max_score.max(a.max(b) as f64);
        if a > b {
            self.wins_a += 1;
        } else if b > a {
            self.wins_b += 1;
        }
    }

    fn average(&self, simulations: u32) -> f64 {
        if simulations > 0 {
            self.total_score / simulations as f64 / 2.0
        } else {
            0.0
        }
    }

    pub(crate) fn print(&self, simulations: u32) {
        println!("\nStatistics:");
        println!("Min score: {}", self.min_score);
        println!("Max score: {}", self.max_score);
        println!("Avg total: {:.2}", self.average(simulations));
        println!("Wins A: {} | Wins B: {}", self.wins_a, self.wins_b);
    }
}