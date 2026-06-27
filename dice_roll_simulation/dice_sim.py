
import random
from matplotlib import pyplot as plt

number_of_rolls = int(input("how many dice rolls do you want to simulate?  "))

simulation_result = {result_key: 0 for result_key in range(2, 13)}

for _ in range(number_of_rolls):
    current_roll = random.randint(1, 6) + random.randint(1, 6)
    simulation_result[current_roll] += 1

possible_outcomes = list(simulation_result.keys())
probabilities = [(simulation_result[outcome] / number_of_rolls) for outcome in possible_outcomes]

plt.figure(figsize=(10, 6))
bar_chart = plt.bar(possible_outcomes, probabilities, color="steelblue", alpha=0.8)
plt.bar_label(bar_chart, padding=3, fmt='p= {:.3f}%')
plt.title(f"Probability of Each Sum When Rolling Two Dice ({number_of_rolls:,} rolls)")
plt.xlabel("Sum of two dice")
plt.ylabel("Probability of outcome")
plt.xticks(possible_outcomes)
plt.grid(axis="y", alpha=0.3)
plt.tight_layout()
plt.show()