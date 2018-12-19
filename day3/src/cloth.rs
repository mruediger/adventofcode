use super::claim;

pub struct Cloth {
    data: Vec<Vec<u32>>,
}

impl Cloth {
    pub fn new(w: usize, h: usize) -> Cloth {
        Cloth {
            data: vec![vec![0; h]; w],
        }
    }

    pub fn mark(&mut self, claim: &claim::Claim) {
        for x in (0..claim.w).map(|i| i + claim.l) {
            for y in (0..claim.h).map(|i| i + claim.t) {
                self.data[x as usize][y as usize] += 1;
            }
        }
    }

    pub fn count_claimed(&self) -> u32 {
        self.data
            .iter()
            .flatten()
            .map(|c| if c > &1 { 1 } else { 0 })
            .sum()
    }
}
