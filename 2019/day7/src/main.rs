extern crate intcode;
extern crate itertools;

use std::fs;

use intcode::Program;
use itertools::Itertools;

const INPUT: &str = "../input.txt";

fn run_program(program: &mut Vec<isize>, input: isize, phase_setting: isize) -> isize {
    let mut program = Program::new(program.to_vec(), vec![input, phase_setting]);

    program.run();
    program.output()[0]
}

fn run_amplifier(program: &mut Vec<isize>, parameters: Vec<isize>) -> isize {
    let output_a = run_program(program, 0, parameters[0]);
    let output_b = run_program(program, output_a, parameters[1]);
    let output_c = run_program(program, output_b, parameters[2]);
    let output_d = run_program(program, output_c, parameters[3]);
    let output_e = run_program(program, output_d, parameters[4]);

    return output_e;
}

#[test]
fn example_1() {
    let mut program = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    assert_eq![43210, run_amplifier(&mut program, vec![4, 3, 2, 1, 0])];
}

#[test]
fn example_2() {
    let mut program = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    assert_eq![54321, run_amplifier(&mut program, vec![0, 1, 2, 3, 4])];
}

#[test]
fn example_3() {
    let mut program = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];

    assert_eq![65210, run_amplifier(&mut program, vec![1, 0, 4, 3, 2])];
}

fn main() {
    let input: Vec<isize> = fs::read_to_string(INPUT)
        .expect("error reading input.txt")
        .split(",")
        .map(|s| s.parse::<isize>())
        .flatten()
        .collect();

    let max_thrust_simple = (0..5)
        .permutations(5)
        .map(|p| run_amplifier(&mut input.clone(), p))
        .max();
    println!("Answer 1: {}", max_thrust_simple.unwrap());
}
