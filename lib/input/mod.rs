use std::fs;
use glob::glob;
use std::io::Read;

fn is_valid_char(c: char, character_set: &str) -> bool {
  character_set.contains(c)
}

pub fn read_input(directory: &str, character_set: &str) -> String {
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
                      .filter(|&c| is_valid_char(c, character_set))
                      .collect::<String>(),
              );
          }
          Err(e) => println!("Error processing file: {:?}", e),
      }
  }

  content
}
