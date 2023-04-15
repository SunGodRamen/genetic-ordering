use key_character_scan_genome::crossover::{pmx_pair_indices};
use key_character_scan_genome::fitness::{fitness};
use key_character_scan_genome::population::{mutate,tournament_selection,initialize_population};
use key_character_scan_genome::input::{read_input};

const POPULATION_SIZE: usize = 32;
const GENERATIONS: usize = 240;
const TOURNAMENT_SIZE: usize = 8;
const CHARACTER_SET: &str = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ1!2@3#4$5%6^7&8*9(0)<.,>-_\\|/?:;=+[{]}`'\"~";

// Genetic algorithm
fn optimize_keyboard(text: &str) -> String {
    let mut population = initialize_population(POPULATION_SIZE, CHARACTER_SET);
    println!("Initial population orderings:");
    for (index, ordering) in population.iter().enumerate() {
        println!("{}: {}", index + 1, ordering);
    }

    for generation in 0..GENERATIONS {
        let mut new_population = Vec::with_capacity(POPULATION_SIZE);

        while new_population.len() < POPULATION_SIZE {
            let parent1 = tournament_selection(&population, text, TOURNAMENT_SIZE, CHARACTER_SET);
            let parent2 = tournament_selection(&population, text, TOURNAMENT_SIZE, CHARACTER_SET);
    
            let mut offspring: Vec<char> = pmx_pair_indices(&parent1, &parent2, CHARACTER_SET).chars().collect();
            mutate(&mut offspring, CHARACTER_SET);
    
            new_population.push(offspring.into_iter().collect());
        }

        population = new_population;

        // Logging the progress
        let best_ordering = population.iter().min_by_key(|ordering| fitness(ordering, text)).unwrap();
        let best_fitness = fitness(&best_ordering, text);
        println!(
            "Generation: {} | Best fitness: {}",
            generation + 1,
            best_fitness
        );

        // Print the orderings
        println!("Current population orderings:");
        for (index, ordering) in population.iter().enumerate() {
            println!("{}: {}", index + 1, ordering);
        }
    }

    population.into_iter().min_by_key(|ordering| fitness(ordering, text)).unwrap()
}

fn main() {
    let directory = "./training-data";
    let text = read_input(directory, CHARACTER_SET);

    let optimized_ordering = optimize_keyboard(&text);
    println!("Optimized ordering: {}", optimized_ordering);
}