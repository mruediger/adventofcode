pub struct InputMap {
    data: Vec<Vec<char>>,
}

impl FromIterator<Vec<char>> for InputMap {
    fn from_iter<I: IntoIterator<Item = Vec<char>>>(iter: I) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        for i in iter {
            data.push(i)
        }

        InputMap { data }
    }
}

impl std::fmt::Display for InputMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in &self.data {
            for c in line {
                write!(f, "{}", c)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl InputMap {

    pub fn new(input: &str) -> Self {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    pub fn from_file(filename: &str) -> Self {
        std::fs::read_to_string(filename).expect("input file")
            .lines().map(|l| l.chars().collect()).collect()
    }

    pub fn get(&self, x: usize, y:usize) -> char {
        self.data[x][y]
    }

    pub fn set(&mut self, x: usize, y:usize, c:char) {
        self.data[x][y] = c;
    }

    pub fn get_adjacent_positions(&self, x:usize, y:usize) -> Vec<(usize, usize)> {
        let x32 = x as i32;
        let y32 = y as i32;
        vec![(  x32,y32-1), (  x32,y32+1),
             (x32+1,y32-1), (x32+1,y32+1), (x32+1,y32),
             (x32-1,y32-1), (x32-1,y32+1), (x32-1,y32)]
            .iter()
            .filter(|(x,y)| *x >= 0 && *y >= 0 && *x < self.data.len() as i32 && *y < self.data[0].len() as i32)
            .map(|&(a, b)| (a as usize, b as usize))
            .collect()
    }

    pub fn get_adjacent_values(&self, x:usize, y:usize) -> Vec<char> {
        self.get_adjacent_positions(x, y)
            .iter()
            .map(|(x,y)| self.data[*x][*y].clone())
            .collect()
    }



    pub fn iter_positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.data.len())
            .flat_map(move |y| {
                let width = self.data.get(y).map(|row| row.len()).unwrap_or(0);
                (0..width).map(move |x| (x,y))
            })
    }
}
