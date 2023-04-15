// Fitness function

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
  }

  total_distance
}

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
