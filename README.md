# Game Simulation Project

This Rust project simulates a probability-based board game involving 12 levers and dice rolls. Players aim to minimize their score by strategically flipping levers down over 5 rounds.

## Game Rules
### base mode:
- **Board**: 12 levers (numbered 1–12), initially all **up**.
- **Rounds**: Each player completes 5 rounds. The lowest total score wins.
- **Turn**:
  1. Roll two 6-sided dice and sum the result.
  2. Flip down lever(s) matching the dice sum:
     - Single lever with the exact number, **or**
     - Multiple levers whose numbers sum to the dice value.
  3. Repeat until no valid moves remain.
- **Scoring**: Sum of **unflipped** lever numbers after each round.

### extended mode:
- **Board**: 12 levers (numbered 1–12), initially all **up**.
- **Turn**: (first player flips down, second up)
  1. Roll two 6-sided dice and sum the result.
  2. Flip lever(s) matching the dice sum:
      - Single lever with the exact number, **or**
      - Multiple levers whose numbers sum to the dice value.
- **End**: When all levers on the board are either up or down

---

## Implementation Overview

### Data Structures
- **Board**: Represented as a 12-bit number (`u16`). Each bit corresponds to a lever (bit 0 = lever 1, ..., bit 11 = lever 12).  
  - `1`: Lever is up.  
  - `0`: Lever is down.  
  - **Operations**: Bitwise AND/OR for flipping, O(1) time complexity.

### Time Complexity
- **Precomputation**: O(1) (static combinations generated once at startup).  
- **Per Turn**: O(1) (lever validity checks use precomputed combinations).  
- **Per Round**: O(k), where `k` = number of turns until no moves remain (varies by dice rolls).  
- **Per Simulation (5 rounds)**: ~5 × O(k).

### Memory Usage
- **Board**: 2 bytes (`u16`).  
- **Precomputed Combinations**: ~200 entries (stored in a `HashMap<u8, Vec<Vec<u8>>>`).  
- **Total per Simulation**: < 1 KB.

---

## In-Depth Implementation

### File Structure

| File                | Description                                                                 |
|---------------------|-----------------------------------------------------------------------------|
| **`board.rs`**      | `Board` struct with bitmask operations (`flip_down_multiple`, `sum_unflipped`). |
| **`dice_throw.rs`** | Simulates rolling two 6-sided dice using `rand`.                           |
| **`strategy.rs`**   | Precomputes valid lever combinations for all dice sums (2–12). Uses greedy selection (highest levers first). |
| **`turn.rs`**       | Executes a single turn: rolls dice, selects levers to flip, updates board. |
| **`round.rs`**      | Runs a full round until no moves remain, returns the unflipped sum.        |
| **`simulation.rs`** | Simulates a full game (5 rounds) and calculates the total score.          |
| **`main.rs`**       | Runs bulk simulations, aggregates results (avg, min, max), and prints metrics. |

---

## Setup & Execution

### Prerequisites
- Rust 1.60+ and Cargo ([Installation Guide](https://www.rust-lang.org/tools/install)).

### Steps
1. **Clone the Project**:
   ```bash
   git clone https://github.com/your-username/game-simulation.git
   cd game-simulation
   ```

2. **Build and Run**:
   ```bash
   cargo run --release
   ```

3. **Run Tests**:
   ```bash
   cargo test
   ```

### Configuration
- Adjust the number of simulations in `main.rs`:
  ```rust
  let num_simulations = 10_000; // Modify this value
  ```

---

## Strategy Design

### Greedy Algorithm
1. **Precomputed Combinations**:
    - All valid lever combinations for sums 2–12 are precomputed at startup.
    - Combinations sorted by:
        - Fewest levers (prioritize single-lever flips).
        - Largest lever values (e.g., `[7]` > `[6, 1]`).

2. **Selection Logic**:
    - For a dice sum, iterate through precomputed combinations in priority order.
    - Choose the first combination where all levers are still **up**.

### Example
- **Dice Sum**: 7
- **Priority Order**:
    1. `[7]` (flip lever 7 if up).
    2. `[6, 1]` (flip 6 and 1 if both up).
    3. `[5, 2]`, etc.

### Efficiency
- **Precomputation**: Done once using `lazy_static!` for fast lookup.
- **No Runtime Calculations**: Immediate selection from sorted combinations.

---

## Future Improvements
- **Alternative Strategies**: Implement probabilistic or DFS-based approaches.
- **Benchmarking**: Compare performance of bitmask vs. alternative data structures.
- **Parallelization**: Use `rayon` to parallelize simulations.