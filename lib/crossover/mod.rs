use rand::Rng;
use rand::{thread_rng};

pub fn row_exchange_crossover(parent1: &str, parent2: &str) -> String {
    let mut rng = thread_rng();
    let crossover_point = rng.gen_range(1..parent1.len());
  
    let parent1_part = &parent1[..crossover_point];
    let parent2_part = &parent2[crossover_point..];
  
    format!("{}{}", parent1_part, parent2_part)
}
  
pub fn cycle_crossover(parent1: &str, parent2: &str) -> String {
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
  
pub fn pmx(parent1: &str, parent2: &str) -> String {
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

pub fn pmx_pair_indices(parent1: &str, parent2: &str, character_set: &str) -> String {
    let parent1_indices = to_indices(parent1, character_set);
    let parent2_indices = to_indices(parent2, character_set);
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

    from_indices(&offspring_indices, character_set)
  }

fn to_indices(ordering: &str, character_set: &str) -> Vec<usize> {
    let mut indices = vec![0; character_set.len()];
    for (i, c) in ordering.chars().enumerate() {
        let idx = character_set.find(c).unwrap();
        indices[idx] = i;
    }
    indices
}

fn from_indices(indices: &[usize], character_set: &str) -> String {
    let mut ordering = vec!['\0'; character_set.len()];
    for (i, &idx) in indices.iter().enumerate() {
        ordering[idx % character_set.len()] = character_set.chars().nth(i).unwrap();
    }
    ordering.into_iter().collect()
}
