use std::io::{self, Write};
use rand::{seq::SliceRandom, thread_rng};

pub fn fitness(ordering: &str, text: &str) -> usize {
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

        let start = if idx >= 6 { idx - 6 } else { 0 };
        let end = if idx + 5 < text.len() { idx + 5 } else { text.len() };
        let context = &text[start..end];

        io::stdout().flush().unwrap(); // Flush the output buffer to display the text
    }

    // Log when the fitness calculation reaches the end of the training data
    total_distance
}


fn find_char_distance(ordering: &str, c1: char, c2: char) -> usize {
    let idx1 = ordering.find(c1).unwrap_or(0);
    let idx2 = ordering.find(c2).unwrap_or(0);
    let len = ordering.len();

    if idx2 == 0 {
        return 4 * len;
    }

    let distance = if idx1 >= idx2 {
        idx1 - idx2
    } else {
        len - (idx2 - idx1)
    };

    distance * 2
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
  
