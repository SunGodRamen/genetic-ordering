//lib/population/mod.rs
use rand::{seq::SliceRandom, thread_rng};

fn create_random_ordering(character_set: &str) -> String {
  let mut rng = thread_rng();

  // Split the character set into pairs
  let pairs: Vec<&str> = character_set.as_bytes().chunks(2).map(|chunk| std::str::from_utf8(chunk).unwrap()).collect();

  // Shuffle the pairs
  let mut shuffled_pairs: Vec<&str> = pairs.clone();
  shuffled_pairs.shuffle(&mut rng);

  // Flatten the shuffled pairs back into a single string
  shuffled_pairs.concat()
}

pub fn initialize_population(population_size: usize, character_set: &str) -> Vec<String> {
  let mut population = Vec::with_capacity(population_size);

  for _ in 0..population_size {
      population.push(create_random_ordering(character_set));
  }

  population
}
