Comprehensive Game-Theoretic Analysis of a Stochastic Bit-Flipping Dice Game with Multi-Round Scoring

## 1. Introduction

This document provides a rigorous game-theoretic study of a novel dice game played on a 12-bit board. Players alternate rolling two dice and strategically flipping bits to minimize (or maximize) their score over multiple rounds. We integrate probabilistic analysis, strategic modeling, and computational methods to explore optimal play and equilibria.

## 2. Game Description

### 2.1 Base Game Rules

1. **Board Setup:** A 12-bit board is initialized to all 1s (hex 0xFFF).
2. **Turns:** On each turn, a player rolls two six-sided dice, summing to a value between 2 and 12.
3. **Bit Flips:** The player flips one of the following:

   * The single bit matching the dice sum, or
   * A subset of bits whose values sum to the dice roll.
4. **Continuation:** The player continues flipping until no valid move remains.
5. **Scoring:** When a player can no longer move, their round score equals the sum of remaining 1-bits.
6. **Rounds:** Reset the board and repeat for five rounds per player.
7. **Victory Condition:** The player with the lower cumulative score after five rounds wins.

### 2.2 Extended Game Rules

Building on the base game, the extended version introduces opposing objectives:

1. **Roles:** Player A clears bits (1→0); Player B sets bits (0→1).
2. **Moves:** On their turn, each player flips either a bit matching their role or a subset summing to the dice roll.
3. **End Conditions:** The game ends when all bits are 0 (Player A wins) or all bits are 1 (Player B wins).

**Key Insight:** Role reversal creates strategic tension over high-value bits. Probability of flipping a particular bit depends on direct rolls or subset decompositions (see Section 4).

## 3. Game-Theoretic Framework

* **Players:** N = {A, B}
* **Actions:** Flip a single bit or a subset summing to the dice roll.
* **States:** S = {0,1}¹² (board configurations); cumulative scores tracked over rounds.
* **Transitions:** Governed by dice‐roll probabilities and chosen flips.
* **Payoffs:** Cumulative negative remaining bits (Base) or binary win/lose (Extended).
* **Classification:** Finite‐horizon, stochastic, perfect‐information, zero‐sum game.

## 4. Probabilistic Analysis

### 4.1 Dice-Sum Probabilities

| Sum (r) | Ways | P(r)          |
| ------- | ---- | ------------- |
| 2, 12   | 1    | 1/36 ≈ 0.0278 |
| 3, 11   | 2    | 2/36 ≈ 0.0556 |
| 4, 10   | 3    | 3/36 ≈ 0.0833 |
| 5, 9    | 4    | 4/36 ≈ 0.1111 |
| 6, 8    | 5    | 5/36 ≈ 0.1389 |
| 7       | 6    | 6/36 ≈ 0.1667 |

### 4.2 Effective Flip Probabilities (P\_eff)

Each bit b (1–12) can be flipped either by rolling b directly or via subset decomposition of a larger roll. Below we list the total P\_eff(b) and principal sources:

| Bit b | Direct P | Subset Sources               | P\_subset | P\_eff ≈  |
| ----- | -------- | ---------------------------- | --------- | --------- |
|  12   |  1/36    | —                            |  0        |  0.0278   |
|  11   |  2/36    | 12→(11+1)                    |  1/36     |  0.0833   |
|  10   |  3/36    | 11→(10+1), 12→(10+2)         |  3/36     |  0.1667   |
|  9    |  4/36    | 10→(9+1), 11→(9+2), 12→(9+3) |  6/36     |  0.2778   |
|  8    |  5/36    | subsets from 9–12            |  10/36    |  0.4167   |
|  7    |  6/36    | subsets from 8–12            |  15/36    |  0.5833   |
|  1–6  | variable | subsets from above sum       | see note  |  see note |

> **Note:** For b < 7, optimal decomposition yields P\_eff(b) ≈ 0.5833 for 2–6, and 0.9722 for b = 1 by splitting all rolls ≥3.

### 4.3 Strategic Flip Value V(b)

Define V(b) = b × P\_eff(b) as the expected score impact.  Bits with higher V(b) are most valuable to target.

| b   | P\_eff  | V(b)    |
| --- | ------- | ------- |
|  12 |  0.0278 |  0.3336 |
|  11 |  0.0833 |  0.9163 |
|  10 |  0.1667 |  1.6670 |
|  9  |  0.2778 |  2.5002 |
|  8  |  0.4167 |  3.3336 |
|  7  |  0.5833 |  4.0831 |
|  6  |  0.5833 |  3.4998 |
|  5  |  0.5833 |  2.9165 |
|  4  |  0.5833 |  2.3332 |
|  3  |  0.5833 |  1.7499 |
|  2  |  0.5833 |  1.1666 |
|  1  |  0.9722 |  0.9722 |

**Insight:** Mid-range bits (7–9) maximize V(b), guiding strategic priorities in both base and extended play.

## 5. Strategic Analysis

*(unchanged summary of base/subset/extended strategies)*

## 6. Mathematical Modeling & Multi-Round Extension

### 6.1 Single-Round MDP

Model each round as an MDP over s∈{0,1}¹².  The Bellman equation for base play:

$$
V^*(s) = \min_{a∈A(s)}\Bigl[c(s,a) + \sum_{r=2}^{12}P(r)·V^*(s_{r,a})\Bigr],
$$

where c(s,a) is immediate cost and s\_{r,a} the post-action state.

### 6.2 Multi-Round State Augmentation

Augment state to (s,t,k) with t=cumulative score, k=round index (1–5):

$$
V^*(s,t,k) = \min_{a∈A(s)}\Bigl[c(s,a) + \sum_{r=2}^{12}P(r)·V^*(s_{r,a},\,t+c(s,a),\,k+\mathbf{1}_{\{s_{r,a}\text{ terminal}\}})\Bigr].
$$

Boundary: V^\*(·,·,6)=t.

### 6.3 Computational Implications

* **Dimensionality:** ≈4096 states×(\~390 max score)×5 rounds → infeasible exact DP.
* **Approximations:** Use function approximation φ(s,t,k)^Tθ and approximate DP or reinforcement learning.

## 7. Equilibrium Concepts & Solution Approaches

### 7.1 Mixed-Strategy Nash

Players choose distributions over flip actions A(s).  Solve

$$
\max_x\min_y x^T U y\quad\text{s.t. }x,y\ge0,\,∑x_i=∑y_j=1,
$$

with U built from expected values V^\*.

### 7.2 Subgame-Perfect via Backward Induction

Treat dice as chance nodes: solve final-round values V^\*(·,·,5), then inductively compute for k=4→1.

### 7.3 Evolutionary Stability & Learning

Define fitness f(σ,Σ) as expected win rate.  σ^\* is ESS if

$$
f(σ^*,σ^*)>f(σ,σ^*)\;∀σ≠σ^*\quad\text{or}\quad f(σ^*,σ^*)=f(σ,σ^*)\text{ and }f(σ^*,σ)>f(σ,σ).
$$

Evolve strategy frequencies via replicator dynamics:

$$
\dot{x_i}=x_i[(U x)_i - x^T U x].
$$

## 8. Computational Approaches

*(unchanged)*

## 9. Empirical Validation

*(unchanged)*

## 10. Future Directions

*(unchanged)*

## Appendix A: Subset-Division Algorithm

*(Pseudocode and examples.)*
