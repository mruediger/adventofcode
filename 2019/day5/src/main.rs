use std::fs;

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

fn run_program(mut program: Vec<isize>, parameter: isize) -> isize {
    let mut ip = 0;
    let mut result = 0;

    loop {
        let x = program[ip];
        let opcode = parse_opcode(x as usize);

        match opcode[3] {
            1 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                dbg!(
                    ip,
                    program[ip],
                    program[ip + 1],
                    program[ip + 2],
                    program[ip + 3],
                    input_a,
                    input_b
                );
                write(&mut program, opcode[0], ip + 3, input_a + input_b);

                ip += 4;
            }
            2 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                dbg!(
                    ip,
                    program[ip],
                    program[ip + 1],
                    program[ip + 2],
                    program[ip + 3],
                    input_a,
                    input_b
                );
                write(&mut program, opcode[0], ip + 3, input_a * input_b);
                ip += 4;
            }
            3 => {
                dbg!(ip, program[ip], program[ip + 1]);
                write(&mut program, opcode[0], ip + 1, parameter);
                ip += 2;
            }
            4 => {
                dbg!(ip, program[ip], program[ip + 1]);
                result = read(&program, opcode[2], ip + 1);
                ip += 2;
            }
            5 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                dbg!(
                    ip,
                    program[ip],
                    program[ip + 1],
                    program[ip + 2],
                    input_a,
                    input_b
                );

                if input_a != 0 {
                    ip = input_b as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                dbg!(
                    ip,
                    program[ip],
                    program[ip + 1],
                    program[ip + 2],
                    input_a,
                    input_b
                );

                if input_a == 0 {
                    ip = input_b as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                dbg!("lt");
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                if input_a < input_b {
                    write(&mut program, opcode[0], ip + 3, 1);
                } else {
                    write(&mut program, opcode[0], ip + 3, 0);
                }
                ip += 4;
            }
            8 => {
                dbg!(
                    ip,
                    program[ip],
                    program[ip + 1],
                    program[ip + 2],
                    program[ip + 3]
                );
                let input_a = read(&program, opcode[2], ip + 1);
                let input_b = read(&program, opcode[1], ip + 2);

                if input_a == input_b {
                    write(&mut program, opcode[0], ip + 3, 1);
                } else {
                    write(&mut program, opcode[0], ip + 3, 0);
                }
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

    if result == 0 {
        program[result as usize]
    } else {
        result
    }
}

#[test]
fn test_day2() {
    let program = "1,9,10,3,2,3,11,0,99,30,40,50"
        .split(",")
        .map(|s| s.parse::<isize>())
        .flatten()
        .collect();

    assert_eq!(run_program(program, 0), 3500);
}

#[test]
fn test_extended_opcode() {
    assert_eq!(parse_opcode(12345), [1, 2, 3, 45]);
    assert_eq!(parse_opcode(1), [0, 0, 0, 1]);
    assert_eq!(parse_opcode(99), [0, 0, 0, 99]);
}

#[test]
fn test_example_1() {
    let program = "1002,4,3,4,33"
        .split(",")
        .map(|s| s.parse::<isize>())
        .flatten()
        .collect();

    assert_eq!(run_program(program, 0), 1002);
}

#[test]
fn test_example_2() {
    let input: Vec<isize> = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
        .split(",")
        .map(|s| s.parse::<isize>())
        .flatten()
        .collect();

    assert_eq!(run_program(input.clone(), 0), 999);
    assert_eq!(run_program(input.clone(), 10), 1001);
}

fn main() {
    let input: Vec<isize> = fs::read_to_string(INPUT)
        .expect("error reading input.txt")
        .split(",")
        .map(|s| s.parse::<isize>())
        .flatten()
        .collect();

    println!("Answer 1: {}", run_program(input.clone(), 1));
    println!("Answer 2: {}", run_program(input.clone(), 5));
}
