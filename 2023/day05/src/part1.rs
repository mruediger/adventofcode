use std::collections::HashMap;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil_map: HashMap<usize, usize>,
    soil_to_fertilizer_map: HashMap<usize, usize>,
    fertilizer_to_water_map: HashMap<usize, usize>,
    water_to_light_map: HashMap<usize, usize>,
    light_to_temperature_map: HashMap<usize, usize>,
    temperature_to_humidity_map: HashMap<usize, usize>,
    humidity_to_location_map: HashMap<usize, usize>,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let seeds: Vec<usize> = input
            .lines()
            .find(|s| !s.is_empty())
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .flat_map(|s| s.parse())
            .collect();

        let mut seed_to_soil_map: HashMap<usize, usize> = HashMap::new();
        let mut soil_to_fertilizer_map: HashMap<usize, usize> = HashMap::new();
        let mut fertilizer_to_water_map: HashMap<usize, usize> = HashMap::new();
        let mut water_to_light_map: HashMap<usize, usize> = HashMap::new();
        let mut light_to_temperature_map: HashMap<usize, usize> = HashMap::new();
        let mut temperature_to_humidity_map: HashMap<usize, usize> = HashMap::new();
        let mut humidity_to_location_map: HashMap<usize, usize> = HashMap::new();
        let mut map = &mut seed_to_soil_map;

        let parse = |s: &str| {
            let vec = s
                .to_string()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
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
                    for i in 0..length {
                        map.insert(s_start + i, d_start + i);
                    }
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

    fn get_location<'a>(&'a self, seed: &'a usize) -> &usize {
        let get_default = |map: HashMap<usize, usize>, i: &usize| {
            let binding = map.get(i).clone();
            *binding.unwrap_or(i)
        };

        let soil: &usize = self.seed_to_soil_map.get(seed).unwrap_or(seed);
        let fertilizer = self.soil_to_fertilizer_map.get(soil).unwrap_or(soil);
        let water = self
            .fertilizer_to_water_map
            .get(fertilizer)
            .unwrap_or(fertilizer);
        let light = self.water_to_light_map.get(water).unwrap_or(water);
        let temperature = self.light_to_temperature_map.get(light).unwrap_or(light);
        let humidity = self
            .temperature_to_humidity_map
            .get(temperature)
            .unwrap_or(temperature);
        self.humidity_to_location_map
            .get(humidity)
            .unwrap_or(humidity)
    }
}

pub fn run(input: &str) {
    let almanac = Almanac::new(input);

    println!("parsing done");

    let seeds = almanac
        .seeds
        .iter()
        .map(|s| almanac.get_location(s))
        .collect::<Vec<_>>();
    println!("{:?}", seeds);
}

#[test]
fn test_get_location() {
    let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    let almanac = Almanac::new(input);
    assert_eq!(almanac.seed_to_soil_map[79], 81);
    assert_eq!(almanac.seed_to_soil_map[14], 14);
    assert_eq!(almanac.seed_to_soil_map[55], 57);
    assert_eq!(almanac.seed_to_soil_map[13], 13);
}
