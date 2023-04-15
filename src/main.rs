use rand::Rng;
use std::fs;
use glob::glob;
use std::io::Read;
use rand::{seq::SliceRandom, thread_rng};

const POPULATION_SIZE: usize = 32;
const GENERATIONS: usize = 240;
const TOURNAMENT_SIZE: usize = 8;
const CHARACTER_SET: &str = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ01!2@3#4$5%6^7&8*9(0)<.,>-_\\|/?:;=+[{]}`'\"~";

fn is_valid_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c.is_whitespace() || ".,-\\/;=[]`".contains(c)
}

fn read_input(directory: &str) -> String {
    let mut content = String::new();

    // Iterate over all text files in the directory
    for entry in glob(&format!("{}/*.txt", directory)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // Read the content of the file
                let mut file_content = String::new();
                fs::File::open(&path)
                    .expect("Failed to open file")
                    .read_to_string(&mut file_content)
                    .expect("Failed to read file");

                // Filter out invalid characters and concatenate the content
                content.push_str(
                    &file_content
                        .chars()
                        .filter(|&c| is_valid_char(c))
                        .collect::<String>(),
                );
            }
            Err(e) => println!("Error processing file: {:?}", e),
        }
    }

    content
}

// Fitness function
fn find_char_distance(ordering: &str, c1: char, c2: char) -> usize {
    let idx1 = ordering.find(c1).unwrap_or(0);
    let idx2 = ordering.find(c2).unwrap_or(0);
    let len = ordering.len();

    if idx2 == 0 {
        return 16 * len;
    }

    let distance = if idx1 >= idx2 {
        idx1 - idx2
    } else {
        len - (idx2 - idx1)
    };

    distance * 4
}


fn fitness(ordering: &str, text: &str) -> usize {
    let mut total_distance = 0;

    // Iterate through the text characters
    for (idx, current_char) in text.char_indices().skip(1) {
        let prev_char = text.chars().nth(idx - 1).unwrap();

        // If the current character is the same as the previous character, travel distance is 0
        if current_char == prev_char {
            continue;
        }

        // Calculate the distance between the current and previous characters in the ordering
        total_distance += find_char_distance(ordering, prev_char, current_char);
    }

    total_distance
}

fn create_random_ordering() -> String {
    let mut rng = thread_rng();
    let mut characters: Vec<char> = CHARACTER_SET.chars().collect();
    characters.shuffle(&mut rng);
    characters.into_iter().collect()
}

fn initialize_population() -> Vec<String> {
    let mut population = Vec::with_capacity(POPULATION_SIZE);

    for _ in 0..POPULATION_SIZE {
        population.push(create_random_ordering());
    }

    population
}

fn tournament_selection(population: &[String], text: &str, k: usize) -> String {
    // Randomly select k orderings from the population
    let mut rng = thread_rng();
    let contestants = population.choose_multiple(&mut rng, k).collect::<Vec<_>>();

    // Find the best ordering among the contestants
    let best_ordering = contestants
        .iter()
        .min_by_key(|ordering| fitness(ordering, text))
        .unwrap();

    best_ordering.to_string()
}

// Row exchange crossover
fn row_exchange_crossover(parent1: &str, parent2: &str) -> String {
    let mut rng = thread_rng();
    let crossover_point = rng.gen_range(1..parent1.len());

    let parent1_part = &parent1[..crossover_point];
    let parent2_part = &parent2[crossover_point..];

    format!("{}{}", parent1_part, parent2_part)
}

fn cycle_crossover(parent1: &str, parent2: &str) -> String {
    let mut offspring = vec!['\0'; parent1.len()];
    let mut visited = vec![false; parent1.len()];

    let mut start_idx = 0;
    while start_idx < parent1.len() && visited[start_idx] == false {
        let mut idx = start_idx;
        while !visited[idx] {
            visited[idx] = true;
            offspring[idx] = parent1.chars().nth(idx).unwrap();
            idx = parent2.find(offspring[idx]).unwrap_or(0);
        }
        start_idx += 1;
    }

    offspring.into_iter().collect()
}

// Partially Mapped Crossover (PMX)
fn pmx(parent1: &str, parent2: &str) -> String {
    let mut rng = thread_rng();
    let len = parent1.len();
    let idx1 = rng.gen_range(0..len);
    let idx2 = rng.gen_range(0..len);
    let (min_idx, max_idx) = if idx1 < idx2 {
        (idx1, idx2)
    } else {
        (idx2, idx1)
    };

    let mut offspring = vec!['\0'; len];
    let mut mapped = vec![false; len];

    // Copy the segment from parent1 to offspring
    for i in min_idx..=max_idx {
        offspring[i] = parent1.chars().nth(i).unwrap();
        mapped[parent2.find(offspring[i]).unwrap()] = true;
    }

    // Map the remaining characters from parent2 to offspring
    for i in 0..len {
        if i < min_idx || i > max_idx {
            let mut idx = i;
            while mapped[idx] {
                idx = parent1.find(parent2.chars().nth(idx).unwrap()).unwrap();
            }
            offspring[i] = parent2.chars().nth(idx).unwrap();
            mapped[idx] = true;
        }
    }

    offspring.into_iter().collect()
}

fn pmx_pair_indices(parent1: &str, parent2: &str) -> String {
    let parent1_indices = to_indices(parent1);
    let parent2_indices = to_indices(parent2);
    let len = parent1_indices.len();
    let pair_len = len / 2;
    let mut rng = thread_rng();
    let idx1 = rng.gen_range(0..pair_len);
    let idx2 = rng.gen_range(0..pair_len);
    let (min_idx, max_idx) = if idx1 < idx2 {
        (idx1 * 2, idx2 * 2)
    } else {
        (idx2 * 2, idx1 * 2)
    };

    let mut offspring_indices = vec![0; len];
    let mut mapped = vec![false; len];

    for i in (min_idx..=max_idx).step_by(2) {
        offspring_indices[i] = parent1_indices[i];
        offspring_indices[i + 1] = parent1_indices[i + 1];
        mapped[parent2_indices[i]] = true;
        mapped[parent2_indices[i + 1]] = true;
    }

    for i in (0..len).step_by(2) {
        if i < min_idx || i > max_idx {
            let mut idx = i;
            while mapped[idx] {
                let parent2_idx = parent2_indices[idx];
                idx = parent1_indices.iter().position(|&x| x == parent2_idx).unwrap();
            }
            offspring_indices[i] = parent2_indices[idx];
            offspring_indices[i + 1] = parent2_indices[idx + 1];
            mapped[idx] = true;
            mapped[idx + 1] = true;
        }
    }

    from_indices(&offspring_indices)
}

fn to_indices(ordering: &str) -> Vec<usize> {
    let mut indices = vec![0; CHARACTER_SET.len()];
    for (i, c) in ordering.chars().enumerate() {
        let idx = CHARACTER_SET.find(c).unwrap();
        indices[idx] = i;
    }
    indices
}

fn from_indices(indices: &[usize]) -> String {
    let mut ordering = vec!['\0'; CHARACTER_SET.len()];
    for (i, &idx) in indices.iter().enumerate() {
        ordering[idx % CHARACTER_SET.len()] = CHARACTER_SET.chars().nth(i).unwrap();
    }
    ordering.into_iter().collect()
}

// Mutation
fn mutate(ordering: &mut Vec<char>) {
    let mut rng = thread_rng();
    let idx1 = rng.gen_range(0..ordering.len());
    let idx2 = rng.gen_range(0..ordering.len());

    ordering.swap(idx1, idx2);
}

// Genetic algorithm
fn optimize_keyboard(text: &str) -> String {
    let mut population = initialize_population();

    for generation in 0..GENERATIONS {
        let mut new_population = Vec::with_capacity(POPULATION_SIZE);

        while new_population.len() < POPULATION_SIZE {
            let parent1 = tournament_selection(&population, text, TOURNAMENT_SIZE);
            let parent2 = tournament_selection(&population, text, TOURNAMENT_SIZE);
    
            let mut offspring: Vec<char> = pmx_pair_indices(&parent1, &parent2).chars().collect();
            mutate(&mut offspring);
    
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
    let text = read_input(directory);

    let optimized_ordering = optimize_keyboard(&text);
    println!("Optimized ordering: {}", optimized_ordering);
}