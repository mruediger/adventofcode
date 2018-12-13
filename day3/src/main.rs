use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod claim;

fn intersections(claim: &claim::Claim, claims: &Vec<claim::Claim>) -> Vec<claim::Claim> {
    Vec::new()
}

fn main() -> std::io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    let claims: Vec<claim::Claim> = reader
        .lines()
        .flatten()
        .map(claim::Claim::from_line)
        .collect();

    let intersections = claims
        .iter()
        .flat_map(|c| c.intersections(&claims))
        .flatten();

    intersections.for_each(|i| println!("{:?}", i));

    Ok(())
}
