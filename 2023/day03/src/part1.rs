trait EngineSchematic {
    fn part_numbers(&self) -> Vec<u32>;
    fn get_adjacent_chars(&self, m: i32, n: i32) -> Vec<char>;
}

impl EngineSchematic for Vec<Vec<char>> {
    fn part_numbers(&self) -> Vec<u32> {
        let mut number: u32 = 0;
        let mut numbers = Vec::new();
        let mut is_part = false;

        for m in 0..self.len() {
            for n in 0..self[m].len() {
                let c = self[m][n];
                if c.is_digit(10) {
                    number = number * 10 + c.to_digit(10).unwrap();
                    is_part = is_part
                        || self
                            .get_adjacent_chars(m as i32, n as i32)
                            .iter()
                            .any(|c| c != &'.' && !c.is_digit(10));
                } else if number != 0 {
                    if is_part {
                        numbers.push(number);
                    }
                    is_part = false;
                    number = 0;
                }
            }
        }
        numbers
    }

    fn get_adjacent_chars(&self, m: i32, n: i32) -> Vec<char> {
        let visits: Vec<(i32, i32)> = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        let mut chars: Vec<char> = Vec::new();

        for v in visits {
            let Some(m2) = usize::try_from(m + v.0).ok() else {
                continue;
            };
            let Some(n2) = usize::try_from(n + v.1).ok() else {
                continue;
            };

            if m2 < self.len() && n2 < self[m2].len() {
                chars.push(self[m2][n2])
            }
        }
        chars
    }
}

pub fn run(input: &Vec<Vec<char>>) -> u32 {
    input.part_numbers().iter().sum()
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

    assert_eq!(
        input.part_numbers(),
        vec![467, 35, 633, 617, 592, 755, 664, 598]
    )
}
