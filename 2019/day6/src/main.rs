use std::collections::HashMap;
use std::fs;

const INPUT: &str = "../input.txt";

fn parse_orbit(string: &str) -> (&str, &str) {
    let parts: Vec<&str> = string.split(")").collect();
    let (name, parent) = (parts[1], parts[0]);

    return (name, parent);
}

fn count_orbits(orbitmap: &HashMap<&str, &str>, parent: &str) -> isize {
    if orbitmap.contains_key(parent) {
        1 + count_orbits(orbitmap, orbitmap[parent])
    } else {
        1
    }
}

fn get_parents<'a>(orbitmap: &HashMap<&str, &'a str>, body: &str, parents: &mut Vec<&'a str>) {
    if orbitmap.contains_key(body) {
        let parent = orbitmap[body];
        parents.push(parent);
        get_parents(orbitmap, parent, parents);
    }
}

#[test]
fn test_example1() {
    let orbitmap: HashMap<&str, &str> = vec![
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ]
    .iter()
    .cloned()
    .map(parse_orbit)
    .collect();

    let mut orbits = 0;

    for (name, parent) in &orbitmap {
        orbits += count_orbits(&orbitmap, parent)
    }

    assert_eq!(orbits, 42);
}

fn main() {
    let input = fs::read_to_string(INPUT).expect("error reading input.txt");

    let orbitmap: HashMap<&str, &str> = input.lines().map(parse_orbit).collect();

    let mut orbits = 0;
    for (name, parent) in &orbitmap {
        orbits += count_orbits(&orbitmap, parent)
    }

    println!("Answer 1: {:?}", orbits);

    let mut you_parents: Vec<&str> = Vec::new();
    get_parents(&orbitmap, "YOU", &mut you_parents);

    let mut san_parents: Vec<&str> = Vec::new();
    get_parents(&orbitmap, "SAN", &mut san_parents);

    let your_steps = you_parents
        .iter()
        .take_while(|your_parent| !san_parents.contains(your_parent))
        .count();

    let santas_steps = san_parents
        .iter()
        .take_while(|santas_parent| !you_parents.contains(santas_parent))
        .count();

    println!("Answer 2: {:?}", your_steps + santas_steps);
}
