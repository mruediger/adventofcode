mod input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = input::InputMap::from_file("../input");

    let movable = input.iter_positions()
        .filter(|(x,y)| input.get(*x,*y) == '@')
        .filter(|(x,y)| {
            input.get_adjacent_values(*x,*y).iter().filter(|c| **c == '@').count() < 4
        })
        .count();

    println!("part 1: {}", movable);
}

fn part2() {
    let mut input = input::InputMap::from_file("../input");

    let mut movable: Vec<(usize,usize)> = input.iter_positions()
        .filter(|(x,y)| input.get(*x,*y) == '@')
        .filter(|(x,y)| {
            input.get_adjacent_values(*x,*y).iter().filter(|c| **c == '@').count() < 4
        })
        .collect();

    let mut total = 0;

    while !movable.is_empty() {
        total += movable.len();

        for (x,y) in &movable {
            input.set(*x,*y,'x')
        }

        for (x,y) in &movable {
            input.set(*x,*y,'.')
        }

        movable = input.iter_positions()
        .filter(|(x,y)| input.get(*x,*y) == '@')
        .filter(|(x,y)| {
            input.get_adjacent_values(*x,*y).iter().filter(|c| **c == '@').count() < 4
        })
        .collect();
    }

    println!("part 2: {}", total);
}

#[test]
fn test_sample_input1() {
    use indoc::indoc;

    const INPUT: &str = indoc!{"..@@.@@@@.
                                @@@.@.@.@@
                                @@@@@.@.@@
                                @.@@@@..@.
                                @@.@@@@.@@
                                .@@@@@@@.@
                                .@.@.@.@@@
                                @.@@@.@@@@
                                .@@@@@@@@.
                                @.@.@@@.@."};

    let mut input = input::InputMap::new(INPUT);

    println!("{}", input);

    let movable: Vec<(usize,usize)> = input.iter_positions()
        .filter(|(x,y)| input.get(*x,*y) == '@')
        .filter(|(x,y)| {
            input.get_adjacent_values(*x,*y).iter().filter(|c| **c == '@').count() < 4
        })
        .collect();

    for (x,y) in movable {
        input.set(x,y,'x')
    }

    println!("{}", input)
}

#[test]
fn test_sample_input2() {
    use indoc::indoc;

    const INPUT: &str = indoc!{"..@@.@@@@.
                                @@@.@.@.@@
                                @@@@@.@.@@
                                @.@@@@..@.
                                @@.@@@@.@@
                                .@@@@@@@.@
                                .@.@.@.@@@
                                @.@@@.@@@@
                                .@@@@@@@@.
                                @.@.@@@.@."};

    let mut input = input::InputMap::new(INPUT);

    let mut movable: Vec<(usize,usize)> = input.iter_positions()
        .filter(|(x,y)| input.get(*x,*y) == '@')
        .filter(|(x,y)| {
            input.get_adjacent_values(*x,*y).iter().filter(|c| **c == '@').count() < 4
        })
        .collect();

    let mut total = 0;

    while movable.len() > 0 {
        total += movable.len();

        for (x,y) in &movable {
            input.set(*x,*y,'x')
        }

        println!("{}", input);

        for (x,y) in &movable {
            input.set(*x,*y,'.')
        }

        movable = input.iter_positions()
        .filter(|(x,y)| input.get(*x,*y) == '@')
        .filter(|(x,y)| {
            input.get_adjacent_values(*x,*y).iter().filter(|c| **c == '@').count() < 4
        })
        .collect();
    }

    println!("{}", total);
}
