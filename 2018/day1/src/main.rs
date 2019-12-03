use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::BTreeSet;

const INPUT: &str = "../input.txt";

fn main() {
    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(&file);

    let inputs: Vec<i32> = reader.lines().flatten()
        .map(|s| s.parse::<i32>()).flatten()
        .collect();

    let mut previous = 0;
    let mut frequencies = BTreeSet::new();

    println!("solution to the first question {}", inputs.iter().sum::<i32>());

    for input in inputs.iter().cycle() {
        previous += input;

        if frequencies.contains(&previous) {
            break;
        }  else {
            frequencies.insert(previous);
        }
    }

    println!("solution to the second question {}", previous);
}
