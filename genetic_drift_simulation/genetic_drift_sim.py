import numpy as np
from matplotlib import pyplot as plt
import random

means = []
index = 0

while True:

        means.append([0.5])

        for x in range(100):

                if (means[index][-1] <= 0 or means[index][-1] >= 1):
                        break
                means[index].append(means[index][-1] + (random.random()-0.5) * 0.2)

        for generation in means:
                plt.plot(generation)
        plt.title("random walk")
        plt.show()

        index += 1