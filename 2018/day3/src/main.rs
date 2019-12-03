#[macro_use]
extern crate lazy_static;
extern crate regex;

mod claim;
mod cloth;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let claims: Vec<claim::Claim> = INPUT.lines().map(claim::Claim::from_line).collect();

    let mut cloth = cloth::Cloth::new();

    claims.iter().for_each(|claim| cloth.mark(claim));

    println!("{} square inches are claimed", cloth.count_claimed());

    let mut contested_ids = cloth.contested_ids();
    contested_ids.sort();
    contested_ids.dedup();

    for claim in claims {
        if contested_ids.binary_search(&claim.id).is_err() {
            println!("{} does not overlap", claim.id)
        }
    }
}
