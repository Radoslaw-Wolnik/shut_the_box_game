# Comprehensive Game-Theoretic Analysis of a Stochastic Bit-Flipping Dice Game with Multi-Round Scoring

## 1. Introduction

This document presents an exhaustive game-theoretic analysis of a novel dice game involving bit manipulation on a 12-bit board, with multiple scoring rounds. The game combines elements of chance (dice rolls) with strategic decision-making, creating a complex environment for player interaction. We analyze the game's structure, strategic considerations, and probabilistic elements to determine optimal play strategies and potential equilibria.

// photo of the box of game // ..\"the game ref"\20240901_092748.jpg

// photo of the game // ..\"the game ref"\20240901_095152.jpg

## 2. Game Description

// photo of the rules // ..\"the game ref"\20240901_092915.jpg

### 2.1 Base Game Rules

1. The game is played on a 12-bit board, initially set to all 1s (0xFFF in hexadecimal).
2. Two players take turns by first rolling two dice and then flipping the avaliable numbers as long as they can
Player turn:
 Throw two dice and sum them up (2-12)
 then:
3. On their turn, a player must flip (from 1 to 0) either:
   - a) The bit corresponding to the dice sum, or
   - b)  A combination of bits that sum to the dice roll.
4. A player's turn continues until no valid move is possible.
5. When a player can no longer make a move, their score for the round is the sum of the remaining 1-bits.
6. The board is reset to all 1s, and the other player begins their round.
7. This process is repeated for a total of 5 rounds for each player.
8. The player with the lower total score across all 5 rounds wins.

**Summary**  
Players alternate turns rolling dice sums (2-12), flipping bits or combinations to minimize their score (remaining 1-bits). Duplicate sums are resolved through subset divisions using a greedy algorithm prioritizing higher values.

#### Summary
In this verison we basicly can just roll 2 dice 5 times writing the sum of each throw
After getting the total 5 numbers we add them up and the person with higher number wins

The only valid difficulty is that we can go out of the lover numbers when we roll the same number twice we shouldnt divide it to the valid sum of numbers (2 to 12) as we can use numbers from 2 to 12

The total solution for scoring is:
For unique numbers write down number
For repeating numbers try to divide them so that all numbers are unique 

eg 

| player | thrown sum | to unique | total |
---
| A      | 8, 7, 7, 5, 4  | 8, 7, 5, 4, 6, 1   | 31 |
| B      | 10, 3, 5, 2, 5 | 10, 5, 3, 2, 4, 1 | 25 |
---


The solution to the division problem isnt hard we can just use  simple greedy alhoritm based going from highest numbers but with restrain of uniqness
Even if we wouldnt know all of the numbers from the start the problem is quite easy to solve and the game just shows basic principles of the propablity

### 2.2 Extended Game Rules
This is where things get more exciting

1. The initial setup is identical to the base game.
2. Players alternate objectives: Player A clears bits (1→0), Player B sets bits (0→1)  
3. Players turn the avaliable for them bits by either changing the number or a subset
4. The game ends when either all bits are 0 (Player A wins) or all bits are 1 (Player B wins).

#### Summary 
In here we see that the second player at the begging have advantage as all of the numbers are flipped to their favor besides the ones the first player unflipped

Theoreticly this is the moment where they have the highest chances of winning just by rolling the same number the previous player rolled in the first turn, and depending on number those chances will be higher or lower

Number / chances
12 1/36
11 1/36
10 2/36 etc..

The problem is that as the player has less of the bits/figures to flip in their favour the chances of hitting that perfect number gets lower ususally with thehighest numbers (12, 11, 10, 9, 8) deciding in favour of one player (as the lover numbers are easly flipped by dividing other numbers into sum of multiple)

So in fact (when considering possibilities of dividing numbers) the chances of flipping a number in a turn are
not / 36 but more idk how much i think sth like 6! * 6!
12 1/36 {12}
11 2/36 {11 and 12=(11, 1)} 
1 35/36 As any number: n besides 2 can be divided to 1 and n-1


## 3. Game-Theoretic Framework
Becouse of the varring propability (And frustration expirienced playing it) we will soley focus on the extended version of the game

### 3.1 Game Structure

- **Players**: N = {1, 2}
- **Action Space**: A = {a ∈ ℤ | 1 ≤ a ≤ 12} ∪ {subsets of A that sum to the dice roll}
- **State Space**: S = {0, 1}^12 (4096 possible states) for each turn, S' = ℤ^5 for the overall game score
- **Transition Function**: T: S × A × S → [0, 1], determined by dice roll probabilities and chosen actions
- **Reward Function**: R: S → ℤ, defined as the negative sum of 1-bits at the end of each round

### 3.2 Game Classification

- **Sequential Game**: Players take alternating rounds
- **Stochastic Game**: Outcomes are influenced by random dice rolls
- **Perfect Information**: The complete game state is observable to both players
- **Zero-Sum Game**: One player's gain is exactly balanced by the other's loss

## 4. Probabilistic Analysis

### 4.1 Dice Roll Probabilities
Let P(r) be the probability of rolling a sum r with two six-sided dice:

| Sum (r) | Probability P(r) |
|---------|------------------|
| 2, 12   | 1/36 ≈ 0.0278    |
| 3, 11   | 2/36 ≈ 0.0556    |
| 4, 10   | 3/36 ≈ 0.0833    |
| 5, 9    | 4/36 ≈ 0.1111    |
| 6, 8    | 5/36 ≈ 0.1389    |
| 7       | 6/36 ≈ 0.1667    |

### 4.2 Effective Flip Probabilities  
Considering subset decomposition strategies:  

| Bit | Effective Probability | Key Combinations |  
|-----|-----------------------|-------------------|  
| 12  | 1/36                  | Direct only       |  
| 11  | 3/36                  | 11, 12=11+1       |  
| 10  | 6/36                  | 10, 11=10+1, 12=10+2 |  
| 9   | 10/36                 | 9, 10=9+1, 11=9+2, 12=9+3 |  
| 8   | 15/36                 | 8 + combinations up to 12 |  
| 7   | 21/36                 | 7 + combinations up to 12 |  
| 1   | 35/36                 | All sums ≥3 via 1+(r-1) |  

### 4.3 Strategic Probability Values  
| Bit | Survival Priority | Flip Vulnerability |  
|-----|-------------------|--------------------|  
| 12  | Highest           | 0.0278             |  
| 11  | High              | 0.0833             |  
| 7   | Medium            | 0.5833             |  
| 1   | Lowest            | 0.9722             |  

## 5. Strategic Analysis

### 5.1 Base Game Strategy

1. **Prioritize High-Value Bits**: 
   - Focus on turning off the highest value bits (10, 11, 12) whenever possible, as these contribute most to the score.
   - The expected value calculations in section 4.2 support this strategy, with a slight preference for mid-range high-probability bits.

2. **Subset Strategy**: 
   - When unable to turn off a high-value bit directly, aim to create or use subsets that include high-value bits.
   - Example: If you roll a 9 and bit 9 is off, consider turning off 5 and 4 instead of 6 and 3, as it leaves more high-value bits available for future turns.

3. **Endgame Considerations**:
   - In the late game, turning off any available bit becomes crucial to avoid ending your turn prematurely.
   - The risk of not being able to move increases as more bits are turned off, so sometimes turning off a lower-value bit is preferable to risking no move.

4. **Probability Management**:
   - Keep track of which high-value bits are still on and adjust strategy based on the remaining probabilities.
   - As the game progresses, the relative value of middle bits (6-8) increases due to their higher probability.

### 5.2 Extended Game Strategy

#### Player A (Turning bits off):
1. **High-Value Focus**: Prioritize turning off high-value bits (10-12) to make it harder for Player B to win quickly.
2. **Subset Creation**: Create subsets that allow turning off multiple high-value bits in future turns.
3. **Probability Exploitation**: Focus on bits 6-8 due to their higher probability, especially if high-value bits are already off.
4. **Endgame Strategy**: In the late game, focus on creating situations where any roll will allow turning off the remaining bits.

#### Player B (Turning bits on):
1. **High-Value Protection**: Keep high-value bits (10-12) on as a priority, as they're harder for Player A to turn off.
2. **Subset Denial**: Turn on bits that prevent Player A from creating favorable high-value subsets.
3. **Probability Exploitation**: Ensure bits 6-8 stay on as long as possible due to their higher probability of being rolled.
4. **Forced Move Creation**: Try to create board states where Player A has limited options, forcing them to turn off low-value bits.

### 5.3 Round Strategy

Within each round, players face a stopping problem: they must balance the benefit of continuing to flip bits against the risk of being forced to stop with a high score.

Optimal stopping strategy σ*(s) for state s:
σ*(s) = continue, if E[R(s')|s] < R(s)
        stop, otherwise

Where E[R(s')|s] is the expected reward of the next state given the current state.

### 5.4 Multi-Round Strategy

Players must consider their strategy across multiple rounds, introducing several new strategic elements:

1. **Score Management**: Players may adopt different strategies based on their cumulative score relative to their opponent.
2. **Risk Assessment**: The willingness to take risks may vary depending on the round number and score difference.
3. **Opponent Modeling**: Players may adjust their strategy based on the opponent's past performance and perceived strategy.

## 6. Mathematical Analysis

### 6.1 Markov Chain Model

The game can be modeled as a Markov chain with 2^12 = 4096 states, each representing a unique board configuration.

State transition probabilities P(s' | s, a) depend on:
1. The current state s (board configuration)
2. The action a (which bits to turn on/off)
3. The probability of rolling each sum

The transition matrix T can be defined as:

T(s, s') = Σ_a P(s' | s, a) * π(a | s)

Where π(a | s) is the policy or strategy employed by the player.

### 6.2 Expected Score Calculation

For the base game, we can calculate the expected score reduction for each move:

E[Score Reduction] = Σ (Bit Value * Probability of turning it off)

For example, turning off bit 7 has an expected score reduction of:
7 * (6/36) = 1.167

### 6.3 Optimal Policy

An optimal policy π*(s) for each state s would minimize the expected cumulative score over all rounds for the base game, or maximize the probability of reaching the winning state for the extended game.

This can be computed using dynamic programming techniques such as value iteration:

V*(s) = min_a [R(s,a) + γ * Σ P(s'|s,a) * V*(s')]

Where:
- V*(s) is the optimal value function
- R(s,a) is the immediate reward (score reduction)
- γ is a discount factor (typically 1 for finite-horizon games)
- P(s'|s,a) is the transition probability

### 6.4 Bellman Equation for Multi-Round Game

For the multi-round game, we can define a Bellman equation that takes into account the cumulative score and round number:

V*(s, score, round) = max_a [R(s, score, a) + γ * Σ_{s', score'} P(s', score' | s, score, a) * V*(s', score', round+1)]

This equation captures the recursive nature of the decision-making process across multiple rounds.

## 7. Game-Theoretic Equilibria

### 7.1 Nash Equilibrium

Given the increased complexity from multiple rounds, a pure strategy Nash equilibrium is unlikely. A mixed strategy equilibrium may exist, where players probabilistically choose between different strategic approaches based on the game state and round number.

To find a Nash equilibrium, we can use the following linear programming formulation:

Maximize v
Subject to:
Σ_i x_i * a_ij ≥ v for all j
Σ_i x_i = 1
x_i ≥ 0 for all i

Where x_i represents the probability of choosing strategy i, and a_ij is the payoff for strategy i against strategy j.

### 7.2 Subgame Perfect Equilibrium

The game can be modeled as an extensive-form game with chance nodes, where each round is a subgame. A subgame perfect equilibrium could theoretically be computed using backward induction, considering all possible score combinations after each round.

The process would involve:
1. Solving for optimal strategies in the final round for all possible score combinations.
2. Working backwards, determining optimal strategies for earlier rounds given the known outcomes of later rounds.

### 7.3 Evolutionary Stable Strategies

In the context of multiple rounds, we can define an Evolutionary Stable Strategy (ESS) as follows:

1. **Strategy Space**: Σ = {Round Strategies} × {Multi-Round Strategies}
2. **Fitness Function**: f(σ, Σ) = Expected win rate of strategy σ against the population Σ across all 5 rounds
3. **ESS Condition**: A strategy σ* is ESS if:
   f(σ*, σ*) > f(σ, σ*) for all σ ≠ σ*, or
   f(σ*, σ*) = f(σ, σ*) and f(σ*, σ) > f(σ, σ) for all σ ≠ σ*

## 8. Computational Complexity

### 8.1 State Space Complexity

The state space now includes both the board state and the cumulative scores:
|S| * |S'| = 4096 * M^10, where M is the maximum possible score per round (78).

### 8.2 Game Tree Complexity

The game tree complexity is significantly increased due to the multiple rounds. An upper bound on the game tree complexity is O((4096 * M^10)^5), where 5 is the number of rounds.

This exponential complexity makes exact computation of optimal strategies infeasible for the full game, necessitating the use of approximation methods.

## 9. Approximate Solutions

Given the increased complexity, approximate solutions become crucial:

### 9.1 Monte Carlo Tree Search (MCTS)

MCTS can be adapted to handle the multi-round structure:
1. **Selection**: Use UCB1 formula to select nodes to explore
2. **Expansion**: Add new board states and score combinations
3. **Simulation**: Randomly play out games to completion
4. **Backpropagation**: Update node values based on simulation results

The search tree depth would correspond to both in-round decisions and round transitions.

### 9.2 Reinforcement Learning

Techniques like Q-learning or policy gradients can be effective in learning strategies that balance in-round and across-round considerations.

Q-learning update rule:
Q(s,a) ← Q(s,a) + α[r + γ max_a' Q(s',a') - Q(s,a)]

Where:
- Q(s,a) is the estimated value of taking action a in state s
- α is the learning rate
- r is the immediate reward
- γ is the discount factor
- s' is the next state

Policy gradient methods can be used to directly learn a policy π(a|s) that maximizes the expected cumulative reward:

∇J(θ) = E[∇ log π_θ(a|s) Q_π(s,a)]

Where:
- J(θ) is the expected cumulative reward
- π_θ(a|s) is the policy parameterized by θ
- Q_π(s,a) is the action-value function under policy π

### 9.3 Heuristic Evaluation Functions

Develop heuristic functions to estimate the value of a given board state and score difference:

H(s, score_diff, round) = w1 * bit_value(s) + w2 * score_diff + w3 * (5 - round)

Where:
- bit_value(s) is the sum of the values of bits still on
- w1, w2, w3 are weights to be tuned

### 9.4 Approximate Dynamic Programming

Use function approximation techniques to estimate the value function:

V(s, score, round) ≈ φ(s, score, round)^T θ

Where:
- φ(s, score, round) is a feature vector
- θ is a weight vector to be learned

Update rule:
θ ← θ + α[r + γ max_a φ(s', score', round+1)^T θ - φ(s, score, round)^T θ] φ(s, score, round)

## 10. Empirical Analysis

Large-scale simulations can provide insights into various aspects of the game:

### 10.1 Simulation Setup

1. Implement multiple strategies:
   - Random
   - Greedy (Highest Value)
   - Probabilistic (Based on dice roll probabilities)
   - MCTS
   - Reinforcement Learning (Q-learning and Policy Gradient)
   - Heuristic-based

2. Run tournaments with 10,000+ games for each strategy matchup

### 10.2 Metrics to Analyze

1. Win rates for different strategies
2. Average game length for the extended version
3. Distribution of scores in the base game
4. Impact of going first vs. second in the extended game
5. Correlation between in-game decisions and overall game outcome

### 10.3 Statistical Analysis

1. Use ANOVA to compare the performance of different strategies
2. Perform regression analysis to identify factors that contribute most to winning
3. Apply clustering algorithms to group similar game states and outcomes

### 10.4 Visualization

1. Create heatmaps of win rates for strategy matchups
2. Plot score distributions for different strategies
3. Visualize game state transitions using directed graphs

## 11. Advanced Topics

### 11.1 Information Set Analysis

Although the game has perfect information, we can analyze information sets in the context of opponent modeling:

I(p, h) = {h' ∈ H : p cannot distinguish between h and h'}

Where:
- I(p, h) is the information set for player p at history h
- H is the set of all possible game histories

### 11.2 Counterfactual Regret Minimization

Adapt CFR for the multi-round structure:

R_T^i(I) = max_a (Σ_t=1^T r_t^i(I, a)) - Σ_t=1^T r_t^i(I, σ_t^i(I))

Where:
- R_T^i(I) is the cumulative regret for player i at information set I
- r_t^i(I, a) is the counterfactual regret for action a at time t

### 11.3 Bayesian Game Analysis

Model uncertainty in opponent strategy as a Bayesian game:

G = (N, Ω, (A_i), (T_i), (p_i), (u_i))

Where:
- Ω is the set of possible opponent types
- T_i is player i's type space
- p_i is player i's prior belief about opponent types

### 11.4 Mechanism Design

Explore variations of the game rules to achieve specific properties:

1. Balanced first-player advantage
2. Increased strategic depth
3. Reduced variance in outcomes

## 12. Future Research Directions

1. Develop and test AI players that can effectively manage both in-round and across-round strategies
2. Investigate how human players handle the increased complexity and whether they develop intuitive heuristics for multi-round play
3. Analyze the impact of different scoring systems on optimal strategies and game balance
4. Explore the application of deep reinforcement learning techniques (e.g., AlphaZero methodology) to the game
5. Conduct a user study to assess the game's appeal, learning curve, and strategic depth from a player's perspective
6. Investigate the potential for creating a meta-game by allowing players to choose or construct their strategies before play
7. Analyze the game's potential as a benchmark for AI decision-making in stochastic, multi-round environments

## 13. Conclusion

This comprehensive analysis provides a thorough examination of the stochastic bit-flipping dice game from a game-theoretic perspective. By combining probabilistic analysis, strategic considerations, mathematical modeling, and computational approaches, we have laid the groundwork for understanding the game's complexity and developing optimal strategies.

The multi-round nature of the game introduces significant strategic depth, requiring players to balance short-term tactical decisions with long-term planning. The stochastic elements add another layer of complexity, necessitating probabilistic reasoning and risk management.

While exact solutions are computationally intractable due to the game's complexity, various approximation methods offer promising avenues for developing strong playing strategies. Empirical analysis through large-scale simulations will be crucial in validating theoretical insights and refining strategic approaches.

Future research in this area has the potential to not only improve play in this specific game but also contribute to broader understanding of decision-making in complex, stochastic environments with multiple rounds of interaction.