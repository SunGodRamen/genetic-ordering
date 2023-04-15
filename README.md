### Keyboard Layout Optimizer
This is a genetic algorithm-based keyboard layout optimizer written in Rust. The goal of this project is to optimize the keyboard layout for a given set of text files, minimizing the total travel distance of fingers while typing the text.

## Features
Reads input text from a directory of text files.
Filters out invalid characters.
Generates an optimized keyboard layout based on a genetic algorithm.
Supports different crossover and mutation operations.

# Dependencies
To run this project, you need to have Rust installed. The project uses the following external crates:
 - rand
 - glob

# How to Run
Clone this repository.
Place your input text files in the ./training-data directory. The algorithm will use these files to optimize the keyboard layout.
Run the following command to build and run the project:
```c
cargo run
```

After the algorithm finishes, you will see the optimized keyboard layout printed in the console.

## Code Overview
Here's a brief overview of the main functions in the code:

 - read_input(directory: &str) -> String: Reads input text files from the given directory and concatenates their contents.
 - is_valid_char(c: char) -> bool: Determines if a character is valid for the algorithm.
 - find_char_distance(ordering: &str, c1: char, c2: char) -> usize: Calculates the distance between two characters in the given ordering.
 - fitness(ordering: &str, text: &str) -> usize: Calculates the fitness score of an ordering based on the total travel distance in the given text.
 - initialize_population() -> Vec<String>: Initializes the population of random orderings.
 - tournament_selection(population: &[String], text: &str, k: usize) -> String: Selects the best ordering from a random sample of the population.
 - row_exchange_crossover(parent1: &str, parent2: &str) -> String: Performs row exchange crossover on two parent orderings.
 - cycle_crossover(parent1: &str, parent2: &str) -> String: Performs cycle crossover on two parent orderings.
 - pmx(parent1: &str, parent2: &str) -> String: Performs partially mapped crossover (PMX) on two parent orderings.
 - mutate(ordering: &mut Vec<char>): Mutates an ordering by swapping two characters.
 - optimize_keyboard(text: &str) -> String: Runs the genetic algorithm to optimize the keyboard layout based on the given text.

# License
This project is released under the MIT License.