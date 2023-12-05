#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
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
    d_start: i64,
    s_start: i64,
    length: i64,
}

impl Conversion {
    fn convert(&self, source: i64) -> Option<i64> {
        let min = self.s_start;
        let max = self.s_start + self.length;
        let offset: i64 = self.d_start - self.s_start;
        match source {
            x if x < min => None,
            x if x >= max => None,
            _ => Some(source + offset),
        }
    }
}

#[derive(Default, Debug)]
struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn get(&self, source: i64) -> i64 {
        self.get_destination(source)
    }

    fn get_destination(&self, source: i64) -> i64 {
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
                .collect::<Vec<i64>>();
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

    fn get_location(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil_map.get(seed);
        let fertilizer = self.soil_to_fertilizer_map.get(soil);
        let water = self.fertilizer_to_water_map.get(fertilizer);
        let light = self.water_to_light_map.get(water);
        let temperature = self.light_to_temperature_map.get(light);
        let humidity = self.temperature_to_humidity_map.get(temperature);
        self.humidity_to_location_map.get(humidity)
    }
}

pub fn run(input: &str) -> i64 {
    let almanac = Almanac::new(input);

    println!("parsing done");

    let mut seeds = almanac
        .seeds
        .iter()
        .map(|s| almanac.get_location(*s))
        .collect::<Vec<_>>();

    seeds.sort();
    seeds[0]
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
    assert_eq!(almanac.seed_to_soil_map.get(79), 81);
    assert_eq!(almanac.seed_to_soil_map.get(14), 14);
    assert_eq!(almanac.seed_to_soil_map.get(55), 57);
    assert_eq!(almanac.seed_to_soil_map.get(13), 13);
}

#[test]
fn test_map() {
    let mut seed_to_soil_map = Map::default();
    seed_to_soil_map.conversions.push(Conversion {
        d_start: 50,
        s_start: 98,
        length: 2,
    });
    seed_to_soil_map.conversions.push(Conversion {
        d_start: 52,
        s_start: 50,
        length: 48,
    });

    assert_eq!(seed_to_soil_map.get_destination(79), 81);
    assert_eq!(seed_to_soil_map.get_destination(14), 14);
    assert_eq!(seed_to_soil_map.get_destination(55), 57);
    assert_eq!(seed_to_soil_map.get_destination(13), 13);
}
