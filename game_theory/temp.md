Comprehensive Game‑Theoretic Analysis of a Stochastic Bit‑Flipping Dice Game with Multi‑Round Scoring

## 1. Introduction

This document provides a rigorous game‑theoretic study of a novel dice game played on a 12‑bit board. Players alternate rolling two dice and strategically flipping bits to minimize (or maximize) their score over multiple rounds. We integrate probabilistic analysis, strategic modeling, and computational methods to explore optimal play and equilibria.

## 2. Game Description

### 2.1 Base Game Rules

1. **Board Setup:** A 12‑bit board is initialized to all 1s (hex 0xFFF).
2. **Turns:** On each turn, a player rolls two six‑sided dice, summing to a value between 2 and 12.
3. **Bit Flips:** The player flips one of the following:

   * The single bit matching the dice sum, or
   * A subset of bits whose values sum to the dice roll.
4. **Continuation:** The player continues flipping until no valid move remains.
5. **Scoring:** When a player can no longer move, their round score equals the sum of remaining 1‑bits.
6. **Rounds:** Reset the board and repeat for five rounds per player.
7. **Victory Condition:** The player with the lower cumulative score after five rounds wins.

**Note:** Detailed subset‑division algorithms are discussed in Section 5.2.

### 2.2 Extended Game Rules

Building on the base game, the extended version introduces opposing objectives:

1. **Roles:** Player A clears bits (1→0); Player B sets bits (0→1).
2. **Moves:** On their turn, each player flips either a bit matching their role or a subset summing to the dice roll.
3. **End Conditions:** The game ends when all bits are 0 (Player A wins) or all bits are 1 (Player B wins).

**Key Insight:** Role reversal creates strategic tension over high‑value bits. Probability of flipping a particular bit depends on direct rolls or subset decompositions (see Section 4.2).

## 3. Game‑Theoretic Framework

* **Players:** N = {A, B}
* **Actions:** Flip a single bit or a subset summing to the dice roll.
* **States:** S = {0,1}^12 (board configurations); cumulative scores tracked over rounds.
* **Transitions:** Governed by dice‑roll probabilities and chosen flips.
* **Payoffs:** Cumulative negative remaining bits (Base) or binary win/lose (Extended).
* **Classification:** Finite‑horizon, stochastic, perfect‑information, zero‑sum game.

## 4. Probabilistic Analysis

### 4.1 Dice‑Sum Probabilities

| Sum   | Probability |
| ----- | ----------- |
| 2, 12 | 1/36        |
| 3, 11 | 2/36        |
| 4, 10 | 3/36        |
| 5, 9  | 4/36        |
| 6, 8  | 5/36        |
| 7     | 6/36        |

### 4.2 Effective Flip Probabilities

Each bit’s flip probability accounts for direct hits and valid subset decompositions. Detailed values and combinatorial breakdowns are provided in Section 8.1.

## 5. Strategic Analysis

### 5.1 Base Game Strategy

1. **High‑Value Priority:** Target bits 10–12 first to maximize score reduction.
2. **Subset Utilization:** When direct flips are unavailable, form subsets including high‑value bits.
3. **Endgame Play:** Late turns may require flipping lower bits to avoid losing turn.
4. **Probability Tracking:** Adjust tactics based on remaining bit probabilities.

### 5.2 Subset‑Division Heuristics

A greedy algorithm ensures unique component sums:

1. Sort desired flips in descending order.
2. For repeated sums, split into the largest possible unique pair.
3. Iterate until all components are distinct.

*(Example and pseudocode moved to Appendix A.)*

### 5.3 Extended Game Strategy

#### Player A (Clearing Bits)

* Focus on denying Player B high‑value bits.
* Create board states limiting B’s subset options.
* Exploit probabilities of mid‑range sums (6–8).

#### Player B (Setting Bits)

* Preserve high bits to force A into low‑value flips.
* Deny A favorable subsets by strategic bit restoration.
* Leverage frequent sums to incrementally rebuild the board.

## 6. Mathematical Modeling

### 6.1 Markov Chain Representation

Model each round as a Markov decision process over 2^12 states, with transition matrix incorporating dice probabilities and action policies.

### 6.2 Dynamic Programming and Bellman Equations

Derive the optimal policy via value iteration:

$V^*(s) = \min_a \Bigl[ r(s,a) + \sum_{s'} P(s'|s,a) V^*(s') \Bigr].$

Extended multi‑round policy uses cumulative score as state augmentation (see Section 6.3).

## 7. Equilibrium Concepts

* **Nash Equilibrium:** Likely mixed; solvable via linear programming.
* **Subgame Perfect Equilibrium:** Obtainable by backward induction across rounds.
* **Evolutionary Stable Strategy:** Defined over repeated tournament play.

## 8. Computational Approaches

### 8.1 Exact vs. Approximate

* **Exact:** Infeasible for full game due to exponential state‑tree growth.
* **Approximate:** Monte Carlo Tree Search, Q‑learning, and policy gradients.

### 8.2 Heuristics and Feature‑Based Methods

Construct heuristic evaluation functions and approximate dynamic programming schemes for tractability.

## 9. Empirical Validation

Design large‑scale simulations (≥10,000 games) comparing strategies on metrics such as win rate, score distribution, and first‑move advantage.

## 10. Future Directions

* AI agent development using deep reinforcement learning.
* Human‑subject studies on strategy formation.
* Variants of scoring and mechanic tweaks for balance analysis.

## Appendix A: Subset‑Division Algorithm

*(Pseudocode and detailed examples.)*
