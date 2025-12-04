mod input;

fn main() {
    part1();
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
}
