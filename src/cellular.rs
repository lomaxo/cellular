use std::{mem, usize};
use std::collections::VecDeque;

use rand::Rng;

const MAX_HISTORY: usize = 4000;

#[derive(Default)]
pub struct GridStats {
    births: u64,
    survivors: u64,
    deaths: u64,
}

impl GridStats {
    pub fn get_births(&self) -> u64 {
        return self.births;
    }
    pub fn get_survivors(&self) -> u64 {
        return self.survivors;
    }
    pub fn get_deaths(&self) -> u64 {
        return self.deaths;
    }
    pub fn get_population(&self) -> u64 {
        return self.births + self.survivors;
    }
}

pub struct Grid {
    cells: Vec<Vec<bool>>,
    prev_cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    age: u64, 
    history: VecDeque<u64>,
    // stats: GridStats,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![vec![false; width]; height],
            prev_cells: vec![vec![false; width]; height],
            width,
            height,
            age: 0,
            history: VecDeque::new(),
            // stats: GridStats::default(),
        }
    }

    pub fn get_max_cells(&self) -> u64 {
        return (self.width * self.height) as u64;
    }

    pub fn get_age(&self) -> u64 {
        return self.age
    }

    pub fn get_cells(&self) -> &Vec<Vec<bool>> {
        return &self.cells
    }

    pub fn get_prev_cells(&self) -> &Vec<Vec<bool>> {
        return &self.prev_cells
    }

    pub fn get_history_data(&self) -> &VecDeque<u64> {
        return &self.history;
    }

    pub fn resize_grid(&mut self, width: usize, height: usize) {
        for row in &mut self.cells {
            row.resize(width, false);
        }
        self.cells.resize(height, vec![false; width]);
        for row in &mut self.prev_cells {
            row.resize(width, false);
        }
        self.prev_cells.resize(height, vec![false; width]);
        (self.width, self.height) = (width, height);
    }

    pub fn randomise_grid(&mut self) {
        self.age = 0;
        self.history.clear();
        const SPAWN_RATIO: i32 = 4;
        let mut rng = rand::rng();
        for row in &mut self.cells {
            for c in row {
                *c = rng.random_range(1..=SPAWN_RATIO) == 1;
            }
        }
        self.prev_cells =  vec![vec![false; self.width]; self.height];

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
        self.age = self.age.saturating_add(1);
        self.history.push_back(self.get_stats().get_population());
        if self.history.len() > MAX_HISTORY { self.history.pop_front(); }
        for x in 0..self.width {
            for y in 0..self.height {
                let live_neighbours = self.get_neighbour_count(x, y);
                if self.cells[y][x] {
                    // Currently alive
                    new_grid[y][x] = if live_neighbours < 2 || live_neighbours > 3  { false } else { true };
                }
                else {
                    // // Current dead
                    new_grid[y][x] = live_neighbours == 3;
                }
            }
        }
        self.prev_cells = mem::replace(&mut self.cells, new_grid);   
    }

    pub fn get_stats(&self) -> GridStats {
        let mut stats = GridStats::default();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cells[y][x] {
                    // Alive
                    if self.prev_cells[y][x] {
                        stats.survivors += 1;
                    } else {  
                        stats.births += 1;
                    }
                } else {
                    // Dead
                    if self.prev_cells[y][x] {
                        stats.deaths += 1;
                    }
                }
            }
        }
        stats
    }
}

