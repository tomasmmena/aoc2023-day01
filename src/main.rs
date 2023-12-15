use std::env;
use std::io::{self, BufRead};
use std::fs;


fn main() {
    let path = env::args().nth(1).expect("Missing required argument input.");

    let data: Vec<usize> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(| line | {
            let mut text = line.expect("Could not read line!");
            let number_strings: [(&str, &str); 10] = [
                ("zero", "0"), 
                ("one", "1"), 
                ("two", "2"), 
                ("three", "3"), 
                ("four", "4"), 
                ("five", "5"), 
                ("six", "6"), 
                ("seven", "7"), 
                ("eight", "8"), 
                ("nine", "9")
            ];

            let mut current_index: usize = 0;
            while current_index < text.len() {
                for (number_string, replacement) in number_strings {
                    if current_index + number_string.len() <= text.len()
                        && number_string == &text[current_index..(current_index + number_string.len())] {
                        text.insert_str(current_index, replacement);
                        current_index += 1;  // increment once more to skip the newly inserted char
                        break;
                    }
                }

                current_index += 1;
            }

            
            for (number_string, replacement) in number_strings {
                if let Some(index) = text.find(number_string) {
                    text.insert_str(index, replacement)
                }
            }
            
            let (first, last) = text.chars().into_iter()
                .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                .fold((None, None), |(first, _), digit| (first.or(Some(digit)), Some(digit)));

            first.expect("No number found in line!") * 10 + last.expect("No number found in line!")
        }).collect();
        
    print!("Total is: {}", data.iter().sum::<usize>())
}
