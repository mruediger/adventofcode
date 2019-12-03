use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

static BASE: usize = 'a' as usize;

fn count(line: &String) -> (u32,u32) {
    let mut word: [u32; 26] = [0; 26];

    line.chars()
        .for_each(|c| word[c as usize - BASE] += 1);

    let mut contains_two = 0;
    let mut contains_three = 0;

    for x in &word {
        if *x == 2 {
            contains_two = 1;
        }
        if *x == 3 {
            contains_three = 1;
        }
    }

    (contains_two, contains_three)
}

trait IDChecker {
    fn compare_to_all(&self, lines: &Vec<String>) -> Option<(String,String)>;
    fn compare_to(&self, line: &String) -> Option<(String,String)>;
}

impl IDChecker for String {
    fn compare_to_all(&self, lines: &Vec<String>) -> Option<(String,String)> {
        lines.iter()
            .filter(|l| l != &self)
            .map(|l| self.compare_to(l))
            .flatten()
            .next()
    }

    fn compare_to(&self, line: &String) -> Option<(String,String)> {
        let mut differed_once = false;

        for (a,b) in self.chars().zip(line.chars()) {
            if a != b {
                if differed_once {
                    return None
                } else {
                    differed_once = true
                }
            }
        }

        Some((self.clone(), line.clone()))
    }
}

fn main() -> std::io::Result<()>{
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .flatten()
        .collect();

    let (twos, threes):(u32,u32) = lines.iter()
        .map(count)
        .fold((0,0), |acc,x| (acc.0 + x.0, acc.1 + x.1));

    println!("{} * {} = {}", twos, threes, twos * threes);

    let boxes : String = lines.iter()
        .map(|line| line.compare_to_all(&lines))
        .flatten()
        .map(|(a,b)| {
            a.chars().zip(b.chars())
                .filter(|(a,b)| a == b )
                .map(|t| t.0)
                .collect::<String>()
        })
        .next()
        .unwrap_or(String::new());


    println!("{:?}", boxes);

    Ok(())
}
