#[derive(Default, Debug, PartialEq)]
pub struct InputMap {
    content: Vec<Vec<char>>,
}

impl FromIterator<Vec<char>> for InputMap {
    fn from_iter<I: IntoIterator<Item = Vec<char>>>(iter: I) -> Self {
        let mut content: Vec<Vec<char>> = Vec::new();
        for i in iter {
            content.push(i)
        }

        InputMap { content }
    }
}

impl std::fmt::Display for InputMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in &self.content {
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

    pub fn rows(&self) -> Vec<Vec<char>> {
        let mut result = Vec::new();
        for row in &self.content {
            result.push(row.clone());
        }
        result
    }

    pub fn collumns(&self) -> Vec<Vec<char>> {
        if self.content.is_empty() {
            return Vec::new();
        }
        let mut result = vec![Vec::new(); self.content[0].len()];

        for row in &self.content {
            for (i, c) in row.iter().enumerate() {
                result[i].push(*c);
            }
        }

        result
    }

    pub fn insert_row(&mut self, i: usize, row: Vec<char>) {
        self.content.insert(i, row);
    }

    pub fn insert_collumn(&mut self, i: usize, collumn: Vec<char>) {
        let mut collumn = collumn.clone();

        for row in &mut self.content {
            row.insert(i, collumn.pop().unwrap())
        }
    }

    pub fn push_row(&mut self, row: Vec<char>) {
        self.content.push(row)
    }

    pub fn push_collumn(&mut self, collumn: Vec<char>) {
        let mut collumn: Vec<char> = collumn.clone();
        collumn.reverse();

        if self.content.len() < collumn.len() {
            self.content
                .append(&mut vec![Vec::new(); collumn.len() - self.content.len()]);
        }

        for row in &mut self.content {
            row.push(collumn.pop().unwrap())
        }
    }
}
