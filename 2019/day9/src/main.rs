extern crate intcode;
use intcode::Program;

fn main() {
    let input: Vec<isize> = std::fs::read_to_string("../input.txt")
        .expect("error reading input.txt")
        .split(",")
        .map(|s| s.parse::<isize>())
        .flatten()
        .collect();

    let mut program_1 = Program::new(input.clone(), vec![1]);
    program_1.run();
    println!("Answer 1: {}", program_1.output()[0]);

    let mut program_2 = Program::new(input.clone(), vec![2]);
    program_2.run();
    println!("Answer 2: {}", program_2.output()[0]);
}
