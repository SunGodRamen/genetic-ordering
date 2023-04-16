//lib/crossover/mod.rs
use rand::Rng;
use rand::{thread_rng};
use std::collections::HashMap;

// Performs partially matched crossover (PMX) on two parents using pair indices.
// The function takes two parent strings and a character set string and returns the offspring string.
pub fn pmx_pair_indices(parent1: &str, parent2: &str, character_set: &str) -> Result<String, String> {
    // Convert the parent strings to a vector of indices corresponding to the characters in the character set
    let parent1_indices = to_indices(parent1, character_set)?;
    let parent2_indices = to_indices(parent2, character_set)?;    
    println!("parent1_indices: {:?}", parent1_indices);
    println!("parent2_indices: {:?}", parent2_indices);

    // Select a random pair of indices to use for PMX crossover
    let len = parent1_indices.len();
    let pair_len = len / 2;
    let mut rng = thread_rng();
    let idx1 = rng.gen_range(0..(len - 1) / 2) * 2;
    let idx2 = rng.gen_range(0..(len - 1) / 2) * 2;
    let (min_idx, max_idx) = if idx1 < idx2 {
        (idx1, idx2)
    } else {
        (idx2, idx1)
    };    

    println!("min_idx: {}, max_idx: {}", min_idx, max_idx);

    // Perform PMX crossover on the selected indices
    let mut offspring_indices = vec![0; len];
    let mut mapped = vec![false; len];

    for i in (min_idx..=max_idx).step_by(2) {
        offspring_indices[i] = parent1_indices[i];
        offspring_indices[i + 1] = parent1_indices[i + 1];
        mapped[parent2_indices[i]] = true;
        mapped[parent2_indices[i + 1]] = true;
    }

    println!("offspring_indices after first loop: {:?}", offspring_indices);
    println!("mapped after first loop: {:?}", mapped);

    let mapping = build_mapping(min_idx, max_idx, &parent1_indices, &parent2_indices);

    for i in (0..len).step_by(2) {
        if i < min_idx || i > max_idx {
            let mut idx = i;
            println!("Before get_mapped_index: {}", idx);
            idx = get_mapped_index(idx, &mapping, 0);
            println!("After get_mapped_index: {}", idx);
    
            offspring_indices[i] = parent2_indices[idx];
            offspring_indices[i + 1] = parent2_indices[idx + 1];
            mapped[idx] = true;
            mapped[idx + 1] = true;
        }
    }
    
    println!("offspring_indices after second loop: {:?}", offspring_indices);

    // Convert the offspring indices back to the original character set and return the offspring string
    println!("offspring_indices before from_indices: {:?}", offspring_indices);
    Ok(from_indices(&offspring_indices, character_set))
}

fn build_mapping(min_idx: usize, max_idx: usize, parent1_indices: &[usize], parent2_indices: &[usize]) -> HashMap<usize, usize> {
    let mut mapping = HashMap::new();
    for i in (min_idx..=max_idx).step_by(2) {
        println!("Mapping {} -> {}", parent2_indices[i], parent1_indices[i]);
        println!("Mapping {} -> {}", parent2_indices[i + 1], parent1_indices[i + 1]);
        mapping.insert(parent2_indices[i], parent1_indices[i]);
        mapping.insert(parent2_indices[i + 1], parent1_indices[i + 1]);
    }
    mapping
}

// Recursively gets the mapped index for PMX crossover
fn get_mapped_index(mut idx: usize, mapped: &HashMap<usize, usize>, recursion_depth: usize) -> usize {
    if recursion_depth >= mapped.len() {
        panic!("Circular reference detected, recursion_depth: {}/{}", recursion_depth, mapped.len());
    }
    
    if let Some(&mapped_idx) = mapped.get(&idx) {
        return get_mapped_index(mapped_idx, mapped, recursion_depth + 1);
    }
    
    idx
}

// Converts a string to a vector of indices corresponding to the characters in the character set
fn to_indices(ordering: &str, character_set: &str) -> Result<Vec<usize>, String> {
    let mut indices = vec![0; character_set.len()];
    for (i, c) in ordering.chars().enumerate() {
        match character_set.find(c) {
            Some(idx) => indices[idx] = i,
            None => {
                return Err(format!("Unexpected character '{}' in ordering string", c));
            }
        }
    }
    Ok(indices)
}

// Converts a vector of indices to a string using the original character set
fn from_indices(indices: &[usize], character_set: &str) -> String {
    let mut ordering = vec!['\0'; character_set.len()];
    for (i, &idx) in indices.iter().enumerate() {
        println!("i: {}, idx: {}", i, idx);
        ordering[idx] = character_set.chars().nth(i).unwrap();
    }
    ordering.into_iter().collect()
}

// Mutates a genetic algorithm population by swapping two elements
pub fn mutate(ordering: &mut Vec<char>, character_set: &str) {
    let mut rng = thread_rng();
    let idx1 = rng.gen_range(0..ordering.len());
    let idx2 = rng.gen_range(0..ordering.len());
  
    ordering.swap(idx1, idx2);
}
