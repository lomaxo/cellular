
pub struct Grid {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    pub fn display(&self) {
        for row in &self.cells {
            for c in row {
                print!("{}", if *c {'X'} else {' '})
            }
            println!();
        }
    }

    pub fn get_grid_strings(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for row in &self.cells {
            let mut line: String = String::new();
            for c in row {
                line = line + {if *c {"X"} else {" "}};
            }
            lines.push(line);
        }
        lines
    }

    pub fn randomise_grid(&mut self) {
        for row in &mut self.cells {
            for c in row {
                *c = rand::random();
            }
        }
    }

    fn get_neighbour_count(&self, x: usize, y: usize) -> u32 {
        let mut count = 0;
        for i in (x as i32 - 1)..(x as i32 + 2) {
            for j in (y as i32 - 1)..(y as i32 + 2) {
                if i != x as i32 || j != y as i32 { // Exclude the cell itself
                    let row = (j + self.height as i32) % self.height as i32;
                    let col = (i + self.width as i32) % self.width as i32;

                    if self.cells[row as usize][col as usize] {
                        count += 1
                    }
                }
            }
        }
        count
    }

    pub fn update_generation(&mut self) {
        let mut new_grid = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let live_neighbours = self.get_neighbour_count(x, y);
                if self.cells[y][x] {
                    // Currently alice
                    new_grid[y][x] = if live_neighbours < 2 || live_neighbours > 3  { false } else { true };
                }
                else {
                    // // Current dead
                    new_grid[y][x] = live_neighbours == 3;
                }
            }
        }
        self.cells = new_grid;   
    }
}

