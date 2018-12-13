#[derive(Debug, Clone)]
pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Claim {
    pub fn from_line(line: String) -> Claim {
        let elements: Vec<&str> = line.split(' ').collect();

        let id = elements
            .get(0)
            .unwrap()
            .trim_left_matches('#')
            .parse()
            .unwrap();

        let cordinates: Vec<u32> = elements
            .get(2)
            .unwrap()
            .trim_right_matches(':')
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let size: Vec<u32> = elements.get(3).unwrap()
            .split('x').map(|s| s.parse().unwrap()).collect();

        Claim {
            id: id,
            x: *cordinates.get(0).unwrap(),
            y: *cordinates.get(1).unwrap(),
            w: *size.get(0).unwrap(),
            h: *size.get(0).unwrap(),
        }
    }

    pub fn intersection(&self, claim: &Claim) -> Option<Claim> {
        None
    }

    pub fn intersections(&self, claims: &Vec<Claim>) -> Vec<Option<Claim>> {
        claims
            .iter()
            .filter(|claim| claim.id != self.id)
            .map(|claim| self.intersection(claim))
            .collect()
    }
}

impl PartialEq for Claim {
    fn eq(&self, other: &Claim) -> bool {
        self.id == other.id
    }
}
impl Eq for Claim {}
