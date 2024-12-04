use std::io::{self, BufRead, BufReader};
use std::fs::File;

// struct PuzzleInput {
//     left: Vec<i32>,
//     right: Vec<i32>,
// }

// impl PuzzleInput {
//     fn new() -> Self {
//         PuzzleInput {
//             left: vec![],
//             right: vec![],
//         }
//     }
// }



fn check_report(levels: Vec<i32>) -> bool{
 
    let mut is_valid = true;
  
    // Grab few rows from test data and make a small test for each case
    // One that only increases
    // Onl
    println!("Report: {:?}",levels);
    if levels.len() < 2 {
        return  true;
    }

    let first_diff = levels[1] - levels[0];
    let is_increasing = first_diff > 0;
    
    for i in 0..levels.len() -1 {
        
        // Check that adjecent numbers are +- 3
        let sum = levels[i+1] - levels[i];
        if sum.abs() > 3 || sum.abs() == 0 {
            println!("Levels did not increase or decrease by 1-3");
            is_valid = false;
            return is_valid;
        }
        
       
        if (is_increasing && sum <= 0) || (!is_increasing && sum >= 0) {
            return false;
        }
   
    }
    println!("Is Valid Report: {}", is_valid);
    return is_valid
   
}

fn is_safe_with_dampener(levels: Vec<i32>) -> bool {
    if check_report(levels.clone()) {
        return true;
    }
    
    // Try removing each level once
    for skip_idx in 0..levels.len() {
        let filtered: Vec<i32> = levels.iter()
            .enumerate()
            .filter(|&(i, _)| i != skip_idx)
            .map(|(_, &x)| x)
            .collect();
            
        if check_report(filtered) {
            return true;
        }
    }
    false
}


fn read_puzzle_input(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // let mut puzzle_input = PuzzleInput::new();
    let mut safe_report_counter = 0;
    for line in reader.lines() {
        match line {
            Ok(val) => {
                // This should have each line as a number now
                let values: Vec<i32> = val
                    .split(" ")
                    .map(|x| x.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let is_valid_report = is_safe_with_dampener(values);
                if is_valid_report {
                    safe_report_counter +=1;
                }

            
            
            }
            Err(err) => {
                return Err(err)
            }
        }
    }
    println!("{}",safe_report_counter);
    return Ok(());
}

fn main() {
    // check_report(vec![7,6,4,2,1]);
    // check_report(vec![1,2,7,8,9]);
    // check_report(vec![9,7,6,2,1]);
    // check_report(vec![1,3,2,4,5]);
    // check_report(vec![1,3,6,7,9]);
    read_puzzle_input("/home/fiz/workbench/advent_of_code_2024/day_2/src/puzzle_input.txt");
}
