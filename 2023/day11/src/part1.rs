pub fn run(input: &str) -> u32 {
    let mut galaxy = crate::common::parse(input);
    galaxy.expand(1);
    galaxy
        .pairs()
        .iter()
        .map(|(a, b)| galaxy.distance(a, b))
        .sum()
}
