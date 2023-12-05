use rayon::prelude::*;

pub fn run(input: &str) -> u32 {
    let almanac = crate::common::Almanac::new(input);

    let ranges: Vec<_> = almanac
        .seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();

    ranges
        .par_iter()
        .map(|s| almanac.get_location(*s))
        .min()
        .unwrap()
}

#[test]
fn test_seed_range() {
    let seeds = [79, 14, 55, 13];

    let ranges = seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();
    assert_eq!(ranges.len(), 27);
}
