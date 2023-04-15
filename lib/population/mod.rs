use rand::Rng;
use rand::{seq::SliceRandom, thread_rng};

use crate::fitness::fitness;

fn create_random_ordering(character_set: &str) -> String {
  let mut rng = thread_rng();
  let mut characters: Vec<char> = character_set.chars().collect();
  characters.shuffle(&mut rng);
  characters.into_iter().collect()
}

pub fn initialize_population(population_size: usize, character_set: &str) -> Vec<String> {
  let mut population = Vec::with_capacity(population_size);

  for _ in 0..population_size {
      population.push(create_random_ordering(character_set));
  }

  population
}

pub fn tournament_selection(population: &[String], text: &str, k: usize, character_set: &str) -> String {
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

// Mutation
pub fn mutate(ordering: &mut Vec<char>, character_set: &str) {
  let mut rng = thread_rng();
  let idx1 = rng.gen_range(0..ordering.len());
  let idx2 = rng.gen_range(0..ordering.len());

  ordering.swap(idx1, idx2);
}