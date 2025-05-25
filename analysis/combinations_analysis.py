# Save this as analysis/combination_heatmap.py
import os
import matplotlib.pyplot as plt
import numpy as np

def generate_combinations():
    combinations = {i: [] for i in range(2, 13)}

    def recurse(target, start, current):
        if target == 0:
            combo = sorted(current, reverse=True)
            if combo not in combinations[sum(combo)]:
                combinations[sum(combo)].append(combo)
            return
        if start < 1:
            return

        max_num = min(start, target)
        for i in range(max_num, 0, -1):
            if i not in current:  # Ensure unique numbers
                recurse(target - i, i - 1, current + [i])

    for target in range(2, 13):
        recurse(target, target, [])

    return combinations

def calculate_frequencies(combinations):
    freq = np.zeros((11, 12))  # Sums 2-12 (rows) vs numbers 1-12 (cols)

    for sum_val, combos in combinations.items():
        for combo in combos:
            for num in combo:
                freq[sum_val-2][num-1] += 1  # sum-2 because sums start at 2

    return freq

def plot_heatmap(freq):
    plt.figure(figsize=(12, 8))

    sums = [str(i) for i in range(2, 13)]
    numbers = [str(i) for i in range(1, 13)]

    plt.imshow(freq, cmap='YlOrRd', aspect='auto')
    plt.colorbar(label='Frequency')

    plt.xticks(np.arange(12), numbers)
    plt.yticks(np.arange(11), sums)
    plt.xlabel("Numbers 1-12")
    plt.ylabel("Target Sum")
    plt.title("Number Frequency in Unique Combinations")

    plt.tight_layout()
    plt.savefig('combination_heatmap.png')
    plt.close()

def calculate_total_counts(combinations):
    counts = np.zeros(12)  # Index 0 = number 1, index 11 = number 12

    for sum_val, combos in combinations.items():
        for combo in combos:
            for num in combo:
                counts[num - 1] += 1  # numbers are 1-12, indexes 0-11

    return counts

def plot_barchart(counts):
    plt.figure(figsize=(12, 6))
    numbers = list(range(1, 13))

    bars = plt.bar(numbers, counts, color='dodgerblue', edgecolor='black')

    # Add value labels on top of bars
    for bar in bars:
        height = bar.get_height()
        plt.text(bar.get_x() + bar.get_width() / 2., height,
                 f'{int(height)}',
                 ha='center', va='bottom')

    plt.xlabel("Numbers (1-12)", fontsize=12)
    plt.ylabel("Total Occurrences", fontsize=12)
    plt.title("Frequency of Numbers in All Combinations", fontsize=14)
    plt.xticks(numbers)
    plt.grid(axis='y', linestyle='--', alpha=0.7)

    plt.tight_layout()
    plt.savefig('number_frequency.png')
    plt.close()

def main():
    combinations = generate_combinations()
    # freq = calculate_frequencies(combinations)
    # plot_heatmap(freq)

    counts = calculate_total_counts(combinations)
    plot_barchart(counts)

if __name__ == "__main__":

    print("Generating heatmap...")
    main()
    print("Saved to analysis/combination_heatmap.png")