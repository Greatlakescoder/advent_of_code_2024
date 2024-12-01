use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug)]
struct PuzzleInput {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl PuzzleInput {
    fn new() -> Self {
        PuzzleInput {
            left: vec![],
            right: vec![],
        }
    }
}

fn read_puzzle_input(path: &str) -> Result<PuzzleInput> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut puzzle_input = PuzzleInput::new();
    for line in reader.lines() {
        match line {
            Ok(val) => {
                // This should have each line as a number now
                let values: Vec<i32> = val
                    .split("  ")
                    .map(|x| x.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                puzzle_input.left.push(values[0]);
                puzzle_input.right.push(values[1]);
            
            }
            Err(err) => {
                anyhow::bail!("{}", err)
            }
        }
    }
    return Ok(puzzle_input);
}

fn calculate_distance() {
    let mut parsed_input = read_puzzle_input("/home/hedrickw/advent_of_code/2024/day_1/src/puzzle_input.txt").unwrap();
    parsed_input.left.sort();
    parsed_input.right.sort();
    let mut final_value = 0; 
    while !parsed_input.left.is_empty() {
        let left_min = parsed_input.left.pop().unwrap();
        let right_min = parsed_input.right.pop().unwrap();
        let distance = left_min - right_min;
        final_value += distance.abs();
        
    }
    println!("{}",final_value);
}

fn calculate_similarity_score() {
    // Hashmap time
    let mut parsed_input = read_puzzle_input("/home/hedrickw/advent_of_code/2024/day_1/src/puzzle_input.txt").unwrap();
    // let locations: HashMap<i32,i32> = HashMap::new();
    // Sort so we only have to loop once
    parsed_input.left.sort();
    parsed_input.right.sort();
    let mut final_score = 0; 
    let mut right_counter = 0;
    while !parsed_input.left.is_empty() {
        let left_val = parsed_input.left.pop().unwrap();
        for val in &parsed_input.right {
            if left_val == *val {
                right_counter += 1;
            }
        }
        final_score += left_val * right_counter;
        right_counter =0;
      
       
    }
    print!("{}",final_score);
    

}

fn main() {
    calculate_similarity_score()
    // calculate_distance()
}
