use super::claim;

pub struct Cloth<'a> {
    data: Vec<Vec<Vec<&'a claim::Claim>>>,
}

impl<'a> Cloth<'a> {
    pub fn new() -> Cloth<'a> {
        Cloth {
            data: vec![vec![Vec::new(); 1]; 1],
        }
    }

    pub fn mark(&mut self, claim: &'a claim::Claim) {
        for x in (0..claim.w as usize).map(|i| i + claim.l as usize) {
            for y in (0..claim.h as usize).map(|i| i + claim.t as usize) {
                if self.data.len() < x + 1 {
                    self.data.resize(x + 1, vec![Vec::new(); 1])
                }

                if self.data[x].len() < y + 1 {
                    self.data[x].resize(y + 1, Vec::new())
                }
                self.data[x][y].push(&claim)
            }
        }
    }

    pub fn count_claimed(&self) -> u32 {
        self.data
            .iter()
            .flatten()
            .map(|c| if c.len() > 1 { 1 } else { 0 })
            .sum()
    }

    pub fn contested_ids(&self) -> Vec<u32> {
        self.data
            .iter()
            .flatten()
            .filter(|c| c.len() > 1)
            .flatten()
            .map(|c| c.id)
            .collect()
    }
}
