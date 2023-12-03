struct Gears {
    chars: Vec<Vec<char>>,
}

#[derive(Debug, Eq, PartialEq)]
struct Partnumber {
    start: usize,
    end: usize,
    number: u32,
}

impl Gears {
    fn get_gears(&self) -> Vec<u32> {
        let mut gears = Vec::new();

        for m in 0..self.chars.len() {
            for n in 0..self.chars[m].len() {
                if self.chars[m][n] == '*' {
                    let partns = self.get_adjacent_part_numbers(m, n);
                    if partns.len() == 2 {
                        gears.push(partns.get(0).unwrap() * partns.get(1).unwrap())
                    }
                }
            }
        }
        gears
    }

    fn get_adjacent_part_numbers(&self, m: usize, n: usize) -> Vec<u32> {
        let mut partnumbers: Vec<Partnumber> = Vec::new();

        for (m, n) in self.get_adjacent_coordinates(m, n) {
            let Some(partnumber) = self.get_partnumber(m, n) else {
                continue;
            };

            if !partnumbers.iter().any(|p| *p == partnumber) {
                println!("{}:{} {:?}", m, n, partnumber);
                partnumbers.push(partnumber);
            }
        }

        partnumbers.iter().map(|p| p.number).collect()
    }

    fn get_partnumber(&self, m: usize, n: usize) -> Option<Partnumber> {
        if !self.chars[m][n].is_ascii_digit() {
            return None;
        }

        let (mut start, mut end) = (n, n);
        while start > 0 && self.chars[m][start - 1].is_ascii_digit() {
            start -= 1;
        }

        while end < self.chars[m].len() && self.chars[m][end].is_ascii_digit() {
            end += 1;
        }

        let mut number = 0;
        let mut is_part = false;
        for n in start..end {
            number = number * 10 + self.chars[m][n].to_digit(10).unwrap();
            is_part = is_part
                || self
                    .get_adjacent_coordinates(m, n)
                    .iter()
                    .map(|(m, n)| self.chars[*m][*n])
                    .any(|c| c != '.' && !c.is_ascii_digit());
        }

        Some(Partnumber { start, end, number })
    }

    fn get_adjacent_coordinates(&self, m: usize, n: usize) -> Vec<(usize, usize)> {
        let neighboars: Vec<(i32, i32)> = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        let mut adjacent_coordinates = Vec::new();

        for v in neighboars {
            let Some(m2) = usize::try_from(m as i32 + v.0).ok() else {
                continue;
            };
            let Some(n2) = usize::try_from(n as i32 + v.1).ok() else {
                continue;
            };

            if m2 < self.chars.len() && n2 < self.chars[m2].len() {
                adjacent_coordinates.push((m2, n2))
            }
        }
        adjacent_coordinates
    }
}

pub fn run(input: &Vec<Vec<char>>) -> u32 {
    Gears {
        chars: input.to_vec(),
    }
    .get_gears()
    .iter()
    .sum()
}

#[test]
fn test_part_numbers() {
    let input: Vec<Vec<char>> = vec![
        "467..114..".chars().collect::<Vec<char>>(),
        "...*......".chars().collect::<Vec<char>>(),
        "..35..633.".chars().collect::<Vec<char>>(),
        "......#...".chars().collect::<Vec<char>>(),
        "617*......".chars().collect::<Vec<char>>(),
        ".....+.58.".chars().collect::<Vec<char>>(),
        "..592.....".chars().collect::<Vec<char>>(),
        "......755.".chars().collect::<Vec<char>>(),
        "...$.*....".chars().collect::<Vec<char>>(),
        ".664.598..".chars().collect::<Vec<char>>(),
    ];
    assert_eq!(Gears { chars: input }.get_gears(), vec![16345, 451490])
}
