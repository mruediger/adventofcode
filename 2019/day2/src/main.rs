use std::fs;

const INPUT: &str = "../input.txt";

fn run_program(noun: usize, verb: usize) -> usize {
    let mut program: Vec<usize> = fs::read_to_string(INPUT)
        .expect("error reading input.txt")
        .split(",")
        .map(|s| s.parse::<usize>())
        .flatten()
        .collect();

    program[1] = noun;
    program[2] = verb;

    let mut ip = 0;

    loop {
        let input_a: usize = program[ip + 1];
        let input_b: usize = program[ip + 2];
        let output: usize = program[ip + 3];

        match program[ip] {
            1 => {
                program[output] = program[input_a] + program[input_b];
                ip += 4;
            }
            2 => {
                program[output] = program[input_a] * program[input_b];
                ip += 4;
            }
            99 => {
                break;
            }
            _ => {
                println!("unknown opcode {}", program[ip]);
                break;
            }
        }
    }
    program[0]
}

fn main() {
    println!("{}", run_program(12, 2));

    for noun in 0..99 {
        for verb in 0..99 {
            if 19690720 == run_program(noun, verb) {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}
