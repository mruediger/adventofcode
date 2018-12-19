#[macro_use]
extern crate lazy_static;
extern crate regex;

mod claim;
mod cloth;

static INPUT: &str = include_str!("../input.txt");
pub type Cloth = Vec<Vec<u8>>;
fn main() {
    let claims: Vec<claim::Claim> = INPUT.lines().map(claim::Claim::from_line).collect();

    let (w, h) = claims.iter().fold((0, 0), |acc, claim| {
        (
            std::cmp::max(acc.0, claim.l + claim.w + 1),
            std::cmp::max(acc.1, claim.t + claim.h + 1),
        )
    });

    let mut cloth = cloth::Cloth::new(w as usize, h as usize);
    claims.iter().for_each(|claim| cloth.mark(claim));

    println!("{} square inches are claimed", cloth.count_claimed());

    let mut unintersected: Vec<claim::Claim> = Vec::new();

    let first = claims[0].clone();

    println!("first claim: {:?}", first);

    for claim in claims {
        if claim.intersects(&first) {
            unintersected.push(claim.clone())
        }
    }

    for x in (0..w) {
        for y in (0..w) {
            claims
                .filter(|claim| !claim.contains_point(x, y))
                .for_each(|claim| unintersected.push(claim))
        }
    }
}
