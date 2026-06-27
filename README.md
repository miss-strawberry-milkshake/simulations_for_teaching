This is a collection of simulations I have made for the high school math/science classes I teach

# Dice Roll Simulation

A python script that simulates adding together the results of two dice rolls "n" times and graphs the result, along with the empirical probability of that outcome.

I used this as a "real world check" of the theoretical probabilities we calculated together as a class.
This was also used to show how the more rolls we simulate, the more accurate the empirical prediction becomes.

<img width="1000" height="600" alt="dice_roll_figure" src="https://github.com/user-attachments/assets/44911e90-4b24-4d7d-82ba-218d085feb84" />

# Central Limit Theorem

A Rust program that also simulations rolling two dice and adding their results.
In this case we ran the simulation multiple times, and then graphed the probability of the observed averages to show it always makes a normal distribution. (The Central Limit Theorem)

I wrote this in Rust so we could run the simulation with massive numbers in a reasonable amount of time.

<img width="640" height="480" alt="image" src="https://github.com/user-attachments/assets/732f0b0b-89a9-4449-9f06-d02b9dc626c9" />


# Genetic Drift Simulation

The observant among you will notice this is just a random walk. Turns out that's a fantastic basis for modeling an evolutionary phenomenon called "Genetic Drift"

Genetic Drift is the process of allele frequency changing over time due to random chance.
We used this simulation to observe how this problem is inversely correlated to population size. The smaller the population, the larger the impact random chance has.

The figure shows Alleles of two different genes, and how the frequency of the Alleles (percentage of individuals in the population with that Allele) will change across generations.

<img width="640" height="480" alt="genetic_drift" src="https://github.com/user-attachments/assets/e6381145-5a20-4d93-91ac-3099cd151c24" />
