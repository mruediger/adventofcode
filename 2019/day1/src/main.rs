use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

extern crate math;

use math::round;

const INPUT: &str = "../input.txt";

struct Modul {
    mass: u32,
}

impl Modul {
    pub fn new(string: String) -> Modul {
        Modul {
            mass: string.parse::<u32>().unwrap(),
        }
    }

    pub fn fuel_required(&self) -> u32 {
        return round::floor(self.mass as f64 / 3 as f64, 0) as u32 - 2;
    }

    pub fn fuel_required_total(&self) -> u32 {
        let mut fuel_required_total = self.fuel_required();
        let mut mass_of_fuel = fuel_required_total as i32;

        loop {
            mass_of_fuel = round::floor(mass_of_fuel as f64 / 3 as f64, 0) as i32 - 2;
            if mass_of_fuel < 0 {
                break;
            }
            fuel_required_total += mass_of_fuel as u32;
        }

        fuel_required_total
    }
}

fn main() {
    let file = File::open(INPUT).expect("file not found");
    let reader = BufReader::new(&file);

    let modules: Vec<Modul> = reader.lines().flatten().map(Modul::new).collect();

    println!("{}", modules.iter().map(|m| m.fuel_required()).sum::<u32>());
    println!(
        "{}",
        modules.iter().map(|m| m.fuel_required_total()).sum::<u32>()
    );
}
