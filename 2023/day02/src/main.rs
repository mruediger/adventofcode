use regex::Regex;

#[derive(Default, Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    pub fn new(string: &str) -> Self {
        //        let re_red = Regex::new(r".*([0-9]+) red.*").unwrap();
        //        let re_green = Regex::new(r".*([0-9]+) green.*").unwrap();
        //        let re_blue = Regex::new(r".*([0-9]+) blue.*").unwrap();
        //
        //        re_red.captures(string).)
        let re = Regex::new(r" *([0-9]+) (\w*) *").unwrap();

        let mut draw = Draw::default();

        for part in string.split(',') {
            let Some(caps) = re.captures(part) else {
                continue;
            };

            let (_, [number, color]) = caps.extract();

            match color {
                "red" => draw.red = number.parse::<u32>().unwrap_or_default(),
                "blue" => draw.blue = number.parse::<u32>().unwrap_or_default(),
                "green" => draw.green = number.parse::<u32>().unwrap_or_default(),
                _ => (),
            }
        }
        draw
    }

    fn is_draw_possible(&self, draw: &Draw) -> bool {
        self.red <= draw.red && self.green <= draw.green && self.blue <= draw.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Default, Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    pub fn new(line: &str) -> Self {
        let re = Regex::new(r"Game ([0-9]+): (.+)$").unwrap();

        let Some(caps) = re.captures(line) else {
            return Game::default();
        };

        let mut draws = Vec::new();
        for draw in caps[2].split(';') {
            draws.push(Draw::new(draw))
        }

        Game {
            id: caps[1].parse::<u32>().unwrap(),
            draws,
        }
    }

    pub fn is_game_possible(&self, draw: Draw) -> bool {
        self.draws.iter().all(|d| d.is_draw_possible(&draw))
    }

    pub fn minimal_draw(&self) -> Draw {
        let mut minimal = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };

        for draw in &self.draws {
            if draw.red > minimal.red {
                minimal.red = draw.red;
            }
            if draw.green > minimal.green {
                minimal.green = draw.green;
            }

            if draw.blue > minimal.blue {
                minimal.blue = draw.blue;
            }
        }
        minimal
    }
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    let mut power_sum = 0;

    for line in input {
        let game = Game::new(&line);
        //12 red cubes, 13 green cubes, and 14 blue
        if game.is_game_possible(Draw {
            red: 12,
            green: 13,
            blue: 14,
        }) {
            sum += game.id;
        }

        power_sum += game.minimal_draw().power();
    }

    println!("{}", sum);
    println!("{}", power_sum);
}
