    use std::collections::HashSet;


struct IDDatabase {
    ranges: Vec<(u64,u64)>,
}

impl IDDatabase {


    fn new(input: &str) -> Self {
        let data:Vec<(u64,u64)> = input.lines().map(|l| l.split_once("-").expect("a string containing -"))
            .map(|(start,end)| (start.parse().unwrap(), end.parse().unwrap()))
            .collect();
        IDDatabase{ ranges: data }
    }

    fn is_fresh(&self, id: u64) -> bool {
        self.ranges.iter().any(|(start,end)| id >= *start && id <= *end)
    }

    fn ranges_count(self) -> usize {
        let mut ids:Vec<u64> = self.ranges.into_iter().flat_map(|(start, end)| start..=end).collect();
        ids.sort();
        ids.dedup();
        ids.len()
    }
}


fn main() {
    let input = std::fs::read_to_string("../input")
        .expect("input file");

    let (id_ranges, ingredients) =
        input.split_once("\n\n")
        .expect("id_ranges and ingredients");

    let ids = IDDatabase::new(id_ranges);
    let count = ingredients.lines()
        .flat_map(|l| l.parse::<u64>())
        .filter(|id| ids.is_fresh(*id))
        .count();
    println!("part 1 {}", count);
    println!("part 2 {}", ids.ranges_count());
}

#[test]
fn test_part1() {

    use indoc::indoc;

    const INPUT: &str = indoc!{"3-5
                                10-14
                                16-20
                                12-18

                                1
                                5
                                8
                                11
                                17
                                32"};



    if let Some((id_ranges, ingredients)) = INPUT.split_once("\n\n") {
        let ids = IDDatabase::new(id_ranges);
        ingredients.lines()
            .flat_map(|l| l.parse::<u64>())
            .for_each(|i| println!("{} is fresh? {}", i, ids.is_fresh(i)));
    }
}


#[test]
fn test_part2() {

    use indoc::indoc;
    use std::collections::HashSet;

    const INPUT: &str = indoc!{"3-5
                                10-14
                                16-20
                                12-18

                                1
                                5
                                8
                                11
                                17
                                32"};



    if let Some((id_ranges, ingredients)) = INPUT.split_once("\n\n") {
        let ids = IDDatabase::new(id_ranges);


        let mut seen = HashSet::new();

        ids.ranges.into_iter().flat_map(|(start, end)| start..=end)
            .filter(|i| seen.insert(*i));


    }
}
