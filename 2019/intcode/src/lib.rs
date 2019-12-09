pub struct Program {
    memory: Vec<isize>,
    input: Vec<isize>,
    output: Vec<isize>,
    ip: usize,
    relative_base: isize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    pub fn new(mode: usize) -> ParameterMode {
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("unknown parameter mode: {}", mode),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfNe,
    JumpIfEq,
    LessThan,
    Equals,
    RelativeBaseOffset,
    Halt,
}

impl Opcode {
    pub fn new(opcode: usize) -> Opcode {
        match opcode {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfNe,
            6 => Opcode::JumpIfEq,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            9 => Opcode::RelativeBaseOffset,
            99 => Opcode::Halt,
            _ => panic!("unknown opcode: {}", opcode),
        }
    }
}

fn parse_opcode(opcode: usize) -> (Opcode, ParameterMode, ParameterMode, ParameterMode) {
    (
        Opcode::new(opcode % 100),
        ParameterMode::new((opcode / 100) % 10),
        ParameterMode::new((opcode / 1_000) % 10),
        ParameterMode::new((opcode / 10_000) % 10),
    )
}

impl Program {
    pub fn new(memory: Vec<isize>, input: Vec<isize>) -> Program {
        Program {
            memory: memory,
            input: input,
            output: Vec::new(),
            ip: 0,
            relative_base: 0,
        }
    }

    pub fn output(&self) -> Vec<isize> {
        self.output.clone()
    }

    pub fn ip(&self) -> usize {
        self.ip
    }

    pub fn run(&mut self) -> Vec<isize> {
        while self.ip < self.memory.len() {
            use self::Opcode::*;

            let (opcode, p0, p1, p2) = parse_opcode(self.memory[self.ip] as usize);

            match opcode {
                Add | Mul => {
                    let input_a = self.read(p0, self.ip + 1);
                    let input_b = self.read(p1, self.ip + 2);

                    let result = match opcode {
                        Add => input_a + input_b,
                        Mul => input_a * input_b,
                        _ => unreachable!("wat"),
                    };
                    self.write(p2, self.ip + 3, result);
                    self.ip += 4;
                }

                Input => {
                    let input = self.input.pop().unwrap();
                    self.write(p0, self.ip + 1, input);
                    self.ip += 2;
                }
                Output => {
                    self.output.push(self.read(p0, self.ip + 1));
                    self.ip += 2;
                }
                JumpIfNe => {
                    let input_a = self.read(p0, self.ip + 1);
                    let input_b = self.read(p1, self.ip + 2);

                    if input_a != 0 {
                        self.ip = input_b as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                JumpIfEq => {
                    let input_a = self.read(p0, self.ip + 1);
                    let input_b = self.read(p1, self.ip + 2);

                    if input_a == 0 {
                        self.ip = input_b as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                LessThan => {
                    let input_a = self.read(p0, self.ip + 1);
                    let input_b = self.read(p1, self.ip + 2);

                    if input_a < input_b {
                        self.write(p2, self.ip + 3, 1);
                    } else {
                        self.write(p2, self.ip + 3, 0);
                    }
                    self.ip += 4;
                }
                Equals => {
                    let input_a = self.read(p0, self.ip + 1);
                    let input_b = self.read(p1, self.ip + 2);

                    if input_a == input_b {
                        self.write(p2, self.ip + 3, 1);
                    } else {
                        self.write(p2, self.ip + 3, 0);
                    }
                    self.ip += 4;
                }
                RelativeBaseOffset => {
                    let input = self.read(p0, self.ip + 1);
                    self.relative_base += input;
                    self.ip += 2;
                }
                Halt => break,
            }
        }

        self.output.clone()
    }

    fn read_or_init(&self, idx: usize) -> isize {
        if idx < self.memory.len() {
            self.memory[idx]
        } else {
            0
        }
    }

    fn read(&self, mode: ParameterMode, ip: usize) -> isize {
        match mode {
            ParameterMode::Position => self.read_or_init(self.read_or_init(ip) as usize),
            ParameterMode::Immediate => self.read_or_init(ip),
            ParameterMode::Relative => {
                self.read_or_init((self.relative_base + self.read_or_init(ip)) as usize)
            }
        }
    }

    fn write_or_init(&mut self, idx: usize, value: isize) {
        if idx >= self.memory.len() {
            self.memory.resize(idx + 1, 0);
        }

        self.memory[idx] = value;
    }

    fn write(&mut self, mode: ParameterMode, ip: usize, value: isize) {
        match mode {
            ParameterMode::Position => {
                let idx = self.read_or_init(ip) as usize;
                self.write_or_init(idx, value);
            }
            ParameterMode::Immediate => self.write_or_init(ip, value),
            ParameterMode::Relative => {
                let idx = (self.relative_base + self.read_or_init(ip)) as usize;
                self.write_or_init(idx, value);
            }
        }
    }
}

#[test]
fn test_209() {
    let input = vec![109, 3];
    Program::new(input.clone(), vec![0]).run();
}

#[test]
fn test_opcode() {
    use self::Opcode::*;
    use self::ParameterMode::*;

    assert_eq!((Halt, Relative, Immediate, Position), parse_opcode(01299));
    assert_eq!((Add, Position, Position, Position), parse_opcode(1));
    assert_eq!((Add, Position, Relative, Relative), parse_opcode(22001));
}

#[test]
fn test_input_position_mode() {
    let input = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    assert_eq!(Program::new(input.clone(), vec![0]).run()[0], 0);
    assert_eq!(Program::new(input.clone(), vec![42]).run()[0], 1);
}

#[test]
fn test_input_immediate_mode() {
    let input = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    assert_eq!(Program::new(input.clone(), vec![0]).run()[0], 0);
    assert_eq!(Program::new(input.clone(), vec![42]).run()[0], 1);
}

#[test]
fn test_day5_example_1() {
    let mut program = Program::new(vec![1002, 4, 3, 4, 33], vec![]);
    program.run();
    assert_eq!(program.memory, vec![1002, 4, 3, 4, 99]);
}

#[test]
fn test_day9_example_1() {
    let input = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];

    let mut program = Program::new(input.clone(), vec![]);
    program.run();
    assert_eq!(input, program.output());
}

#[test]
fn test_day9_example_2() {
    let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut program = Program::new(input.clone(), vec![]);
    program.run();
    assert_eq!(16, program.output()[0].to_string().len());
}

#[test]
fn test_day9_example_3() {
    let input = vec![104, 1125899906842624, 99];
    let mut program = Program::new(input.clone(), vec![]);
    program.run();
    assert_eq!(1125899906842624, program.output()[0]);
}
