extern crate itertools;

use std::fs;

use itertools::Itertools;

const INPUT: &str = "../input.txt";

fn parse_opcode(opcode: usize) -> [usize; 4] {
    let mut result = [0, 0, 0, 0];

    result[3] = opcode % 100;
    result[2] = (opcode / 100) % 10;
    result[1] = (opcode / 1_000) % 10;
    result[0] = (opcode / 10_000) % 10;

    result
}

fn read(program: &Vec<isize>, mode: usize, ip: usize) -> isize {
    if mode == 0 {
        program[program[ip] as usize]
    } else {
        program[ip]
    }
}

fn write(program: &mut Vec<isize>, mode: usize, ip: usize, value: isize) {
    if mode == 0 {
        let idx = program[ip] as usize;
        program[idx] = value;
    } else {
        program[ip] = value;
    }
}

fn run_program(program: &mut Vec<isize>, input: isize, phase_setting: isize) -> (isize, bool) {
    let mut ip = 0;
    let mut result = 0;
    let mut halted = false;
    let mut read_once = false;

    loop {
        let x = program[ip];
        let opcode = parse_opcode(x as usize);
        match opcode[3] {
            1 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);
                write(program, opcode[0], ip + 3, input_a + input_b);

                ip += 4;
            }
            2 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                write(program, opcode[0], ip + 3, input_a * input_b);
                ip += 4;
            }
            3 => {
                if read_once {
                    write(program, opcode[0], ip + 1, input);
                } else {
                    write(program, opcode[0], ip + 1, phase_setting);
                    read_once = true;
                }
                ip += 2;
            }
            4 => {
                result = read(&program, opcode[2], ip + 1);
                ip += 2;
            }
            5 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                if input_a != 0 {
                    ip = input_b as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                if input_a == 0 {
                    ip = input_b as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                if input_a < input_b {
                    write(program, opcode[0], ip + 3, 1);
                } else {
                    write(program, opcode[0], ip + 3, 0);
                }
                ip += 4;
            }
            8 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                if input_a == input_b {
                    write(program, opcode[0], ip + 3, 1);
                } else {
                    write(program, opcode[0], ip + 3, 0);
                }
                ip += 4;
            }
            99 => {
                halted = true;
                break;
            }
            _ => {
                println!("unknown opcode {}", program[ip]);
                break;
            }
        }
    }

    (result, halted)
}

fn run_amplifier(program: &mut Vec<isize>, parameters: Vec<isize>) -> isize {
    let (output_a, _) = run_program(program, 0, parameters[0]);
    let (output_b, _) = run_program(program, output_a, parameters[1]);
    let (output_c, _) = run_program(program, output_b, parameters[2]);
    let (output_d, _) = run_program(program, output_c, parameters[3]);
    let (output_e, _) = run_program(program, output_d, parameters[4]);

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
