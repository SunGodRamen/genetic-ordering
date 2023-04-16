//src/main.rs
// Import necessary modules
use key_character_scan_genome::input::{read_input};
use key_character_scan_genome::population::{initialize_population};
use key_character_scan_genome::crossover::{pmx_pair_indices, mutate};
use key_character_scan_genome::fitness::{fitness, tournament_selection};

// Set constants
const POPULATION_SIZE: usize = 4;
const GENERATIONS: usize = 24;
const TOURNAMENT_SIZE: usize = 2;
const CHARACTER_SET: &str = "aAbBcCdDeE";

fn main() {
    // Set the input directory and read the training data
    let directory = "./training-data";
    let text = read_input(directory, CHARACTER_SET);

    // Run the genetic algorithm and print the optimized keyboard ordering
    match optimize_keyboard(&text) {
        Ok(optimized_ordering) => println!("Optimized ordering: {}", optimized_ordering),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Define the main genetic algorithm function
fn optimize_keyboard(text: &str) -> Result<String, String> {
    // Initialize the population with random orderings
    let mut population = initialize_population(POPULATION_SIZE, CHARACTER_SET);
    
    // Print the initial population
    println!("Initial population orderings:");
    for (index, ordering) in population.iter().enumerate() {
        println!("{}: {}", index + 1, ordering);
    }

    // Loop through generations
    for generation in 0..GENERATIONS {
        // Create a new empty population for the next generation
        let mut new_population = Vec::with_capacity(POPULATION_SIZE);

        // Keep generating offspring until the new population is full
        while new_population.len() < POPULATION_SIZE {
            // Select two parents using tournament selection
            let parent1 = tournament_selection(&population, text, TOURNAMENT_SIZE, CHARACTER_SET);
            let parent2 = tournament_selection(&population, text, TOURNAMENT_SIZE, CHARACTER_SET);
    
            // Create offspring using PMX crossover and mutate them
            let mut offspring: Vec<char> = pmx_pair_indices(&parent1, &parent2, CHARACTER_SET)?.chars().collect();
            println!("pmx done");
            mutate(&mut offspring, CHARACTER_SET);
            println!("mutate done");
    
            // Add the offspring to the new population
            new_population.push(offspring.into_iter().collect());
            println!("population push done");
        }

        // Replace the old population with the new one
        population = new_population;

        // Find the best ordering in the current population and print its fitness
        let best_ordering = population.iter().min_by_key(|ordering| fitness(ordering, text)).unwrap();
        let best_fitness = fitness(&best_ordering, text);
        println!(
            "Generation: {} | Best fitness: {}",
            generation + 1,
            best_fitness
        );

        // Print the current population
        println!("Current population orderings:");
        for (index, ordering) in population.iter().enumerate() {
            println!("{}: {}", index + 1, ordering);
        }
    }

    // Return the best ordering found after all generations
    Ok(population.into_iter().min_by_key(|ordering| fitness(ordering, text)).unwrap())
}
