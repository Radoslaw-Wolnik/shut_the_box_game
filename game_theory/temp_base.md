## 6. Mathematical Modeling & Multi-Round Extension

In this section, we translate the game into precise recursive formulas, then explain each component in the context of our bit‑flipping dice game.

### 6.1 Single-Round Markov Decision Process (MDP)

We model one round as an MDP:

$V^*(s) = \min_{a \in A(s)} \Bigl[c(s,a) + \sum_{r=2}^{12} P(r)\,V^*(s_{r,a})\Bigr].$

* **State (s):** A specific 12‑bit configuration (which bits remain set to 1).  For example, s = 0b111111111111 at start.
* **Actions (A(s)):** All legal flips: either flip the bit matching the dice roll or flip a valid subset summing to the roll.  Which actions depend on s.
* **Immediate cost (c(s,a)):** The increase in your score when you take action a.  In the base game, cost = sum of 1‑bits left after flipping — but we can equivalently minimize remaining bits, so c(s,a) is the decrease in that sum.
* **Dice probabilities (P(r)):** The chance of rolling sum r (e.g. P(7)=6/36).
* **Next state (s\_{r,a}):** The board after you flip according to action a in response to roll r.
* **Interpretation:** For each state s, the optimal value V\* is the minimum expected remaining-bits score you can achieve: choose the flip a that trades off your immediate bit reduction c(s,a) against the future cost V\*(s\_{r,a}) averaged over possible rolls.

### 6.2 Multi-Round State Augmentation

To capture all 5 rounds, we augment the state with:

* **t:** Cumulative score (sum of remaining bits) so far.
* **k:** Current round index, k = 1…5.

The recursion becomes:

$V^*(s, t, k) = \min_{a \in A(s)} \Bigl[c(s,a) + \sum_{r=2}^{12} P(r)\,V^*(s_{r,a},\,t + c(s,a),\,k + \mathbf{1}_{\{\text{terminal}(s_{r,a})\}})\Bigr],$

with boundary condition:

$V^*(\cdot, t, 6) = t.$

* **terminal(s'):** A boolean indicating the end of a round (no valid flip remains).
* **k + 1\_{…}:** We advance to the next round only when s\_{r,a} has no legal moves.
* **Boundary:** Once k=6, all five rounds are complete, so value equals total score t.

This formula tells us: from any partially completed game (board s, accumulated t, round k), pick the flip a that minimizes your final total score, accounting for both immediate c(s,a) and the downstream effect of future rounds.

### 6.3 Computational Implications

* **State-Space Size:** 4096 possible boards × up to ≈390 cumulative scores × 5 rounds ≈ millions of states.
* **Exact DP Infeasible:** Too large for brute‑force.
* **Approximations:** Use function‑approximation: represent V(s,t,k)≈φ(s,t,k)ᵀθ with feature vector φ and learn θ via approximate dynamic programming or reinforcement learning (e.g. Q‑learning).  This reduces dimensionality by compressing state.

---

## 7. Equilibrium Concepts & Solution Approaches

Here we discuss three frameworks for finding optimal or stable strategies in our zero-sum, stochastic, multi-round game.

### 7.1 Mixed-Strategy Nash Equilibrium

* **Idea:** At each decision point (state s), rather than choosing a single action, players randomize over actions according to a probability distribution x.
* **Payoff Matrix (U):** For any pair of mixed strategies x (for Player A) and y (for Player B), U(x,y) gives A’s expected total score (B’s score is the negative).
* **Minimax Formulation:** In zero-sum games, A solves

  $\max_x\min_y\; x^T U y, \quad \text{s.t. } x_i, y_j \ge 0, \sum_i x_i = \sum_j y_j =1.$

  Solving this linear program yields equilibrium distributions x\*, y\* that guarantee the best worst-case performance.

### 7.2 Subgame-Perfect Equilibrium via Backward Induction

* **Extensive Form Representation:** Treat each round k and each dice roll as nodes in a game tree, with chance nodes for rolls and decision nodes for flips.
* **Backward Induction:** Start at the final round (k=5) and compute optimal strategies V\*(s, t, 5) for every possible board and score.  Then move backward: use those values to solve for k=4, and so on, until the initial state.  This yields a strategy profile that is optimal in every subgame (i.e., after any history of rolls and flips).

### 7.3 Evolutionary Stable Strategies (ESS) & Learning Dynamics

* **Strategy Population (Σ):** Imagine a large population of players, each adopting a strategy σ.

* **Fitness Function (f):** f(σ,Σ) = expected win rate of σ against the current population mix Σ.

* **ESS Criterion:** A strategy σ\* is evolutionarily stable if, when most players use σ\*, no mutant strategy σ can invade:

  1. f(σ\*,σ\*) > f(σ,σ\*) for all σ ≠ σ\*, or
  2. If f(σ\*,σ\*) = f(σ,σ\*), then f(σ\*,σ) > f(σ,σ).

* **Dynamics:** Strategy frequencies xᵢ evolve according to the replicator equation:

  $\dot x_i = x_i\bigl[(U x)_i - x^T U x\bigr],$

  where (U x)\_i is the expected payoff of pure strategy i against the population mix x.  Over time, this process favors strategies with above‐average fitness, potentially converging to an ESS.

---

*End of Expanded Sections 6 & 7.*
