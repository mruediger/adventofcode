use crate::common::Point;
use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    // get loop
    let sketch = crate::common::parse(input).unwrap();
    let sloop = sketch.get_loop();

    let mut cache = HashMap::new();
    cache.insert((0, 0), true);
    let mut inside = 0_usize;

    for x in 0..sketch.len() {
        for y in 0..sketch.width() {
            if sloop.contains(&(x, y)) {
                print!("X");
                cache.insert((x, y), false);
            } else if is_outside((x, y), &mut cache, &sloop, sketch.len(), sketch.width()) {
                print!(".");
                cache.insert((x, y), true);
            } else {
                cache.insert((x, y), false);
                print!("I");
                inside += 1;
            }
        }
        print!("   {}", cache.len());
        println!();
    }

    inside
}

fn is_outside(
    pos: Point,
    cache: &mut HashMap<Point, bool>,
    sloop: &Vec<Point>,
    len: usize,
    width: usize,
) -> bool {
    if cache.contains_key(&pos) {
        return cache.get(&pos) == Some(&true);
    } else if sloop.contains(&pos) {
        cache.insert(pos, false);
        return false;
    } else if is_border(pos, len, width) {
        cache.insert(pos, true);
        return true;
    } else {
        for neighboar in neighboars(pos, len, width) {
            if is_outside(neighboar, cache, sloop, len, width) {
                cache.insert(neighboar, true);
                return true;
            }
        }
        return false;
    }
}

fn neighboars(pos: Point, len: usize, width: usize) -> Vec<Point> {
    let mut points = Vec::new();

    // up
    if pos.0 > 0 {
        points.push((pos.0 - 1, pos.1))
    }

    // down
    //    if pos.0 < len {
    //        points.push((pos.0 + 1, pos.1))
    //    }

    // left
    if pos.1 > 0 {
        points.push((pos.0, pos.1 - 1));
    }

    // right
    if pos.1 < width {
        points.push((pos.0, pos.1 + 1));
    }

    points
}

fn is_border(pos: Point, len: usize, width: usize) -> bool {
    pos.0 == 0 || pos.0 + 1 == len || pos.1 == 0 || pos.1 + 1 == width
}
