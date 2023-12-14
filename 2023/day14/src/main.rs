use common::input::InputMap;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    tilt_north(RockMap::new(input))
        .rows()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, r)| r.iter().filter(|c| **c == 'O').count() * (i + 1))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut rocks = RockMap::new(input);

    for _ in 0..1000 {
        // works as well
        rocks = cycle(rocks);
    }
    rocks
        .rows()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, r)| r.iter().filter(|c| **c == 'O').count() * (i + 1))
        .sum()
}

type RockMap = InputMap;

fn tilt_north(rocks: RockMap) -> RockMap {
    let mut tilted_rocks = RockMap::default();

    for collumn in rocks.collumns() {
        let mut tmp_collumn: Vec<char> = Vec::new();
        for (i, c) in collumn.iter().enumerate() {
            if c == &'O' {
                tmp_collumn.push(*c);
            } else if c == &'#' {
                // fill empty spaces
                tmp_collumn.append(&mut vec!['.'; i - tmp_collumn.len()]);
                tmp_collumn.push('#');
            }
        }
        tmp_collumn.append(&mut vec!['.'; collumn.len() - tmp_collumn.len()]);
        tilted_rocks.push_collumn(tmp_collumn);
    }

    tilted_rocks
}

fn tilt_south(rocks: RockMap) -> RockMap {
    let mut tilted_rocks = RockMap::default();

    for collumn in rocks.collumns() {
        let mut tmp_collumn: Vec<char> = Vec::new();
        for (i, c) in collumn.iter().enumerate() {
            if c == &'.' {
                tmp_collumn.push(*c);
            } else if c == &'#' {
                // fill empty spaces
                tmp_collumn.append(&mut vec!['O'; i - tmp_collumn.len()]);
                tmp_collumn.push('#');
            }
        }
        tmp_collumn.append(&mut vec!['O'; collumn.len() - tmp_collumn.len()]);
        tilted_rocks.push_collumn(tmp_collumn);
    }

    tilted_rocks
}

fn tilt_west(rocks: RockMap) -> RockMap {
    let mut tilted_rocks = RockMap::default();

    for row in rocks.rows() {
        let mut tmp_row: Vec<char> = Vec::new();
        for (i, c) in row.iter().enumerate() {
            if c == &'O' {
                tmp_row.push(*c);
            } else if c == &'#' {
                // fill empty spaces
                tmp_row.append(&mut vec!['.'; i - tmp_row.len()]);
                tmp_row.push('#');
            }
        }
        tmp_row.append(&mut vec!['.'; row.len() - tmp_row.len()]);
        tilted_rocks.push_row(tmp_row);
    }

    tilted_rocks
}

fn tilt_east(rocks: RockMap) -> RockMap {
    let mut tilted_rocks = RockMap::default();

    for row in rocks.rows() {
        let mut tmp_row: Vec<char> = Vec::new();
        for (i, c) in row.iter().enumerate() {
            if c == &'.' {
                tmp_row.push(*c);
            } else if c == &'#' {
                // fill empty spaces
                tmp_row.append(&mut vec!['O'; i - tmp_row.len()]);
                tmp_row.push('#');
            }
        }
        tmp_row.append(&mut vec!['O'; row.len() - tmp_row.len()]);
        tilted_rocks.push_row(tmp_row);
    }

    tilted_rocks
}

fn cycle(rocks: RockMap) -> RockMap {
    tilt_east(tilt_south(tilt_west(tilt_north(rocks))))
}

#[cfg(test)]
mod tests {

    use indoc::indoc;

    const INPUT: &str = indoc! {"O....#....
                                 O.OO#....#
                                 .....##...
                                 OO.#O....O
                                 .O.....O#.
                                 O.#..O.#.#
                                 ..O..#O..O
                                 .......O..
                                 #....###..
                                 #OO..#...."};

    const TILTED: &str = indoc! {"OOOO.#.O..
                                  OO..#....#
                                  OO..O##..O
                                  O..#.OO...
                                  ........#.
                                  ..#....#.#
                                  ..O..#.O.O
                                  ..O.......
                                  #....###..
                                  #....#...."};

    const CYCLE_1: &str = indoc! {".....#....
                                   ....#...O#
                                   ...OO##...
                                   .OO#......
                                   .....OOO#.
                                   .O#...O#.#
                                   ....O#....
                                   ......OOOO
                                   #...O###..
                                   #..OO#...."};

    const CYCLE_2: &str = indoc! {".....#....
                                   ....#...O#
                                   .....##...
                                   ..O#......
                                   .....OOO#.
                                   .O#...O#.#
                                   ....O#...O
                                   .......OOO
                                   #..OO###..
                                   #.OOO#...O"};

    const CYCLE_3: &str = indoc! {".....#....
                                   ....#...O#
                                   .....##...
                                   ..O#......
                                   .....OOO#.
                                   .O#...O#.#
                                   ....O#...O
                                   .......OOO
                                   #...O###.O
                                   #.OOO#...O"};

    #[test]
    fn tilt() {
        let tilted_rocks = super::tilt_north(super::RockMap::new(INPUT));
        assert_eq!(tilted_rocks, super::RockMap::new(TILTED));
    }

    #[test]
    fn cycle() {
        let mut cycled_rocks = super::cycle(super::RockMap::new(INPUT));
        assert_eq!(cycled_rocks, super::RockMap::new(CYCLE_1));
        cycled_rocks = super::cycle(cycled_rocks);
        assert_eq!(cycled_rocks, super::RockMap::new(CYCLE_2));
        cycled_rocks = super::cycle(cycled_rocks);
        assert_eq!(cycled_rocks, super::RockMap::new(CYCLE_3));
    }

    #[test]
    fn part1() {
        let load: usize = super::part1(INPUT);
        assert_eq!(load, 136);
    }

    #[test]
    fn part2() {
        let load: usize = super::part2(INPUT);
        assert_eq!(load, 64);
    }
}
