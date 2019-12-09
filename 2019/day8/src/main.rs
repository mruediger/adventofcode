use std::fs;

const INPUT: &str = "../input.txt";

struct Picture {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Picture {
    pub fn new(data: Vec<u32>, width: usize, height: usize) -> Picture {
        Picture {
            data,
            width,
            height,
        }
    }

    pub fn layers(&self) -> Vec<Vec<u32>> {
        self.data
            .chunks(self.width * self.height)
            .map(|s| s.to_vec())
            .collect::<Vec<Vec<u32>>>()
    }

    pub fn render(&self) -> Vec<Vec<u32>> {
        self.layers()
            .iter()
            .rev()
            .fold(vec![2; self.width * self.height], |acc, i| merge(acc, i))
            .chunks(self.width)
            .map(|s| s.to_vec())
            .collect::<Vec<Vec<u32>>>()
    }
}

fn merge(layer_a: Vec<u32>, layer_b: &Vec<u32>) -> Vec<u32> {
    layer_a
        .iter()
        .zip(layer_b.iter())
        .map(|(a, b)| if *b == 2 { a } else { b })
        .cloned()
        .collect()
}

#[test]
fn example_2() {
    let picture = Picture::new(vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0], 2, 2);

    assert_eq!(vec![vec![0, 1], vec![1, 0]], picture.render());
}

fn main() {
    let input: Vec<u32> = fs::read_to_string(INPUT)
        .expect("error reading input.txt")
        .chars()
        .map(|c| c.to_digit(10))
        .flatten()
        .collect();

    let picture = Picture::new(input, 25, 6);
    let layers = picture.layers();
    let (ones, twos): (Vec<u32>, Vec<u32>) = layers
        .iter()
        .min_by_key(|l| l.iter().filter(|i| **i == 0).count())
        .unwrap()
        .iter()
        .filter(|i| **i != 1 || **i != 2)
        .partition(|&i| *i == 1);

    let answer_1 = ones.len() * twos.len();

    println!("Answer 1: {:?}", answer_1);

    println!("Answer 2");
    picture.render().iter().for_each(|l| println!("{:?}", l));
}
