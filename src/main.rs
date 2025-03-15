use std::thread;
use std::time::Duration;

struct Grid {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    fn display(&self) {
        for row in &self.cells {
            for c in row {
                print!("{}", if *c {'X'} else {' '})
            }
            println!();
        }
    }

    fn randomise_grid(&mut self) {
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
                if (i != x as i32 || j != y as i32) { // Exclude the cell itself
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

    fn update_generation(&mut self) {
        let mut new_grid = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let live_neighbours = self.get_neighbour_count(x, y);
                if self.cells[y][x] {
                    // Currently alice
                    if live_neighbours < 2 || live_neighbours > 3 {
                        new_grid[y][x] = false;
                    } else {
                        new_grid[y][x] = true;
                    }
                }
                else {
                    // Current dead
                    if live_neighbours == 3 {
                        new_grid[y][x] = true;
                    } else {
                        new_grid[y][x] = false;
                    }
                }
            }
        }
        self.cells = new_grid;   
    }
}


fn main() {
    println!("Hello, world!");
    let mut grid = Grid::new(100,20);
    grid.randomise_grid();
    for i in 0..500 {
        println!("Generation: {i}");
        grid.update_generation();
        grid.display();

        let milliseconds = 50; // Sleep for 500 milliseconds

        // Create a Duration from the milliseconds
        let duration = Duration::from_millis(milliseconds);
    
        // Pause the current thread for the specified duration
        thread::sleep(duration);
        
    }
    
}
