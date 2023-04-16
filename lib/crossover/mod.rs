//lib/crossover/mod.rs
use rand::{thread_rng, Rng};
use std::collections::HashMap;

// Performs partially matched crossover (PMX) on two parents using pair indices.
// PMX is a genetic algorithm crossover technique that preserves relative order of genes.
// The function takes two parent strings and a character set string and returns the offspring string.
pub fn pmx_pair_indices(parent1: &str, parent2: &str, character_set: &str) -> Result<String, String> {

    // Convert the parent strings to a vector of indices corresponding to the characters in the character set
    let parent1_indices = to_indices(parent1, character_set)?;
    let parent2_indices = to_indices(parent2, character_set)?;
    println!("parent1_indices: {:?}", parent1_indices);
    println!("parent2_indices: {:?}", parent2_indices);

    // Select a random pair of indices to use for PMX crossover
    // This will define the "mapping section" of the crossover.
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
    // Start by copying the mapping section from parent1 to the offspring
    let mut offspring_indices = vec![0; len];
    let mut mapped = vec![false; len];

    // Copy the selected indices from parent1 to the offspring
    for i in (min_idx..=max_idx).step_by(2) {
        offspring_indices[i] = parent1_indices[i];
        offspring_indices[i + 1] = parent1_indices[i + 1];
        mapped[parent2_indices[i]] = true;
        mapped[parent2_indices[i + 1]] = true;
    }

    println!("offspring_indices after first loop: {:?}", offspring_indices);
    println!("mapped after first loop: {:?}", mapped);

    // Build the mapping table between the two parents based on the mapping section
    let mapping = build_mapping(min_idx, max_idx, &parent1_indices, &parent2_indices);

    // Use the mapping table to avoid duplicate values when copying non-mapping section genes
    let mut used_indices = std::collections::HashSet::new();
    for i in (min_idx..=max_idx).step_by(2) {
        used_indices.insert(offspring_indices[i]);
        used_indices.insert(offspring_indices[i + 1]);
    }
    
    // For each non-mapping section gene in parent2, check if it's not already in the offspring
    // If it's not, copy the gene to the offspring
    // Otherwise, use the mapping table to find the corresponding gene in parent1 and copy that instead
    for i in (0..len).step_by(2) {
        if i < min_idx || i > max_idx {
            let mut idx = i;
            println!("Before get_mapped_index: {}", idx);
            idx = get_mapped_index(idx, &mapping, 0);
            println!("After get_mapped_index: {}", idx);
    
            // Add the index from parent2 to the offspring if it hasn't been used yet
            if !used_indices.contains(&parent2_indices[idx]) {
                offspring_indices[i] = parent2_indices[idx];
                used_indices.insert(parent2_indices[idx]);
            }
            // Add the next index from parent2 to the offspring if it hasn't been used yet
            if !used_indices.contains(&parent2_indices[idx + 1]) {
                offspring_indices[i + 1] = parent2_indices[idx + 1];
                used_indices.insert(parent2_indices[idx + 1]);
            }
        }
    }

    println!("offspring_indices after second loop: {:?}", offspring_indices);

    // Convert the offspring indices back to the original character set and return the offspring string
    println!("offspring_indices before from_indices: {:?}", offspring_indices);
    Ok(from_indices(&offspring_indices, character_set))
}

// This function builds a mapping table between the parents' indices for PMX crossover
// It takes the minimum and maximum indices defining the mapping section, along with the parents' index vectors
// It returns a hashmap that maps each index from parent2 to its corresponding index in parent1
fn build_mapping(min_idx: usize, max_idx: usize, parent1_indices: &[usize], parent2_indices: &[usize]) -> HashMap<usize, usize> {
    // Create an empty hashmap to store the mapping
    let mut mapping = HashMap::new();
    // Iterate over each pair of indices in the mapping section
    for i in (min_idx..=max_idx).step_by(2) {
        println!("Mapping {} -> {}", parent2_indices[i], parent1_indices[i]);
        println!("Mapping {} -> {}", parent2_indices[i + 1], parent1_indices[i + 1]);

        // Insert the mappings into the hashmap
        mapping.insert(parent2_indices[i], parent1_indices[i]);
        mapping.insert(parent2_indices[i + 1], parent1_indices[i + 1]);
    }

    // Return the completed mapping table
    mapping
}

// Recursively gets the mapped index for PMX crossover
fn get_mapped_index(mut idx: usize, mapped: &HashMap<usize, usize>, recursion_depth: usize) -> usize {
    // If the recursion depth is greater than or equal to the number of elements in the mapping table, there must be a circular reference.
    // In this case, the function panics with an error message.
    if recursion_depth >= mapped.len() {
        panic!("Circular reference detected, recursion_depth: {}/{}", recursion_depth, mapped.len());
    }
    
    // If the index is in the mapping table, get the corresponding mapped index and recursively call this function with the mapped index.
    // This will keep following the mapping table until the original index is reached.
    if let Some(&mapped_idx) = mapped.get(&idx) {
        return get_mapped_index(mapped_idx, mapped, recursion_depth + 1);
    }
    
    // If the index is not in the mapping table, it does not need to be mapped and can be returned as is.
    idx
}

// Converts a string to a vector of indices corresponding to the characters in the character set
fn to_indices(ordering: &str, character_set: &str) -> Result<Vec<usize>, String> {
    // Create a vector of zeros with the same length as the character set.
    // This vector will hold the indices of the characters in the ordering.
    let mut indices = vec![0; character_set.len()];
    for (i, c) in ordering.chars().enumerate() {
        // Find the index of the character in the character set.
        // If the character is not in the character set, return an error.
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
    // Create a vector of null characters with the same length as the character set.
    // This vector will hold the characters in the correct order.
    let mut ordering = vec!['\0'; character_set.len()];
    for (i, &idx) in indices.iter().enumerate() {
        // Get the character corresponding to the index in the character set and add it to the ordering vector.
        ordering[idx] = character_set.chars().nth(i).unwrap();
    }
    // Convert the ordering vector to a string and return it.
    ordering.into_iter().collect()
}

// Mutates a genetic algorithm population by changing one element to a random ASCII character
pub fn mutate(ordering: &mut Vec<char>, _character_set: &str) {
    // Generate a random index within the range of the ordering vector.
    let mut rng = thread_rng();
    let idx = rng.gen_range(0..ordering.len());

    // Generate a random ASCII value within the range of valid ASCII characters (0-127).
    let ascii_value = rng.gen_range(0..=127);
    // Convert the ASCII value to a character and replace the element at the random index with the new character.
    let new_char = char::from_u32(ascii_value).unwrap();
    ordering[idx] = new_char;
}
