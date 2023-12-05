use rayon::prelude::*;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map,
}

#[derive(Debug)]
struct Conversion {
    d_start: u32,
    s_start: u32,
    length: u32,
}

impl Conversion {
    fn convert(&self, source: u32) -> Option<u32> {
        let min = self.s_start;
        let max: u64 = self.s_start as u64 + self.length as u64;
        let offset: i64 = self.d_start as i64 - self.s_start as i64;
        match source {
            x if x < min => None,
            x if x as u64 >= max => None,
            _ => Some((source as i64 + offset) as u32),
        }
    }
}

#[derive(Default, Debug)]
struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn get(&self, source: u32) -> u32 {
        self.get_destination(source)
    }

    fn get_destination(&self, source: u32) -> u32 {
        self.conversions
            .iter()
            .find_map(|c| c.convert(source))
            .unwrap_or(source)
    }
}

impl Almanac {
    fn new(input: &str) -> Self {
        let seeds = input
            .lines()
            .find(|s| !s.is_empty())
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .flat_map(|s| s.parse())
            .collect();

        let mut seed_to_soil_map = Map::default();
        let mut soil_to_fertilizer_map = Map::default();
        let mut fertilizer_to_water_map = Map::default();
        let mut water_to_light_map = Map::default();
        let mut light_to_temperature_map = Map::default();
        let mut temperature_to_humidity_map = Map::default();
        let mut humidity_to_location_map = Map::default();
        let mut map = &mut seed_to_soil_map;

        let parse = |s: &str| {
            let vec = s
                .to_string()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>();
            match &vec[..] {
                &[a, b, c] => (a, b, c),
                _ => (0, 0, 0),
            }
        };

        for line in input.lines().filter(|s| !s.is_empty()).skip(1) {
            match line {
                "seed-to-soil map:" => map = &mut seed_to_soil_map,
                "soil-to-fertilizer map:" => map = &mut soil_to_fertilizer_map,
                "fertilizer-to-water map:" => map = &mut fertilizer_to_water_map,
                "water-to-light map:" => map = &mut water_to_light_map,
                "light-to-temperature map:" => map = &mut light_to_temperature_map,
                "temperature-to-humidity map:" => map = &mut temperature_to_humidity_map,
                "humidity-to-location map:" => map = &mut humidity_to_location_map,
                _ => {
                    let (d_start, s_start, length) = parse(line);
                    map.conversions.push(Conversion {
                        d_start,
                        s_start,
                        length,
                    });
                }
            };
        }

        Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        }
    }

    fn get_location(&self, seed: u32) -> u32 {
        let soil = self.seed_to_soil_map.get(seed);
        let fertilizer = self.soil_to_fertilizer_map.get(soil);
        let water = self.fertilizer_to_water_map.get(fertilizer);
        let light = self.water_to_light_map.get(water);
        let temperature = self.light_to_temperature_map.get(light);
        let humidity = self.temperature_to_humidity_map.get(temperature);
        self.humidity_to_location_map.get(humidity)
    }
}

pub fn run(input: &str) -> u32 {
    let almanac = Almanac::new(input);

    let ranges: Vec<_> = almanac
        .seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();

    ranges
        .par_iter()
        .map(|s| almanac.get_location(*s))
        .min()
        .unwrap()
}

#[test]
fn test_seed_range() {
    let seeds = [79, 14, 55, 13];

    let ranges = seeds
        .chunks(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();
    assert_eq!(ranges.len(), 27);
}
