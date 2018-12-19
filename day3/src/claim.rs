use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Claim {
    pub id: u32,
    pub l: u32,
    pub t: u32,
    pub w: u32,
    pub h: u32,
}

impl Claim {
    pub fn from_line(line: &str) -> Claim {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^#(?P<id>\d+) @ (?P<l>\d+),(?P<t>\d+): (?P<w>\d+)x(?P<h>\d+)$")
                    .unwrap();
        }

        let caps = RE.captures(line).unwrap();

        Claim {
            id: caps["id"].parse::<u32>().unwrap(),
            l: caps["l"].parse::<u32>().unwrap(),
            t: caps["t"].parse::<u32>().unwrap(),
            w: caps["w"].parse::<u32>().unwrap(),
            h: caps["h"].parse::<u32>().unwrap(),
        }
    }

    pub fn intersects(&self, claim: &Claim) -> bool {
        if claim.contains_claim(&self) {
            return true;
        }

        let (a, b, c, d) = (
            (claim.l, claim.t + claim.h),
            (claim.l + claim.w, claim.t + claim.h),
            (claim.l + claim.w, claim.t),
            (claim.l, claim.t),
        );

        self.contains_point(a.0, a.1)
            || self.contains_point(b.0, b.1)
            || self.contains_point(c.0, c.1)
            || self.contains_point(d.0, d.1)
    }

    fn contains_claim(&self, claim: &Claim) -> bool {
        (self.l < claim.l && self.l + self.w > claim.l + claim.w)
            && (self.t < claim.t && self.t + self.h > claim.t + claim.h)
    }

    fn contains_point(&self, x: u32, y: u32) -> bool {
        (self.l < x) && (x < (self.l + self.w)) && (self.t < y) && (y < self.t + self.h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_line() {
        assert_eq!(
            Claim::from_line("#1 @ 509,796: 18x15"),
            Claim {
                id: 1,
                l: 509,
                t: 796,
                w: 18,
                h: 15
            }
        );
    }

    #[test]
    fn test_intersect() {
        let a = Claim {
            id: 1,
            l: 10,
            t: 10,
            w: 5,
            h: 5,
        };

        let b = Claim {
            id: 2,
            l: 12,
            t: 12,
            w: 5,
            h: 5,
        };

        assert!(a.intersects(&b));
        assert!(b.intersects(&a));

        let c = Claim {
            id: 6,
            l: 71,
            t: 733,
            w: 17,
            h: 14,
        };
        let d = Claim {
            id: 25,
            l: 542,
            t: 825,
            w: 22,
            h: 19,
        };
    }
}
