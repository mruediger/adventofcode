pub fn run(input: &str) -> usize {
    let (instructions, nodes) = crate::common::parse_input(input);
    crate::common::count_steps(instructions, "AAA", |s: &str| s == "ZZZ", nodes)
}
