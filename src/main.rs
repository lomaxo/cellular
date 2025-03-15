use std::thread;
use std::time::Duration;

mod cellular;

fn main() {
    println!("Hello, world!");
    let mut grid = cellular::Grid::new(100,20);
    grid.randomise_grid();
    for i in 0..500 {
        println!("Generation: {i}");
        grid.update_generation();
        grid.display();

        // Create a Duration from the milliseconds
        let duration = Duration::from_millis(50);
        // Pause the current thread for the specified duration
        thread::sleep(duration);
        
    }
    
}
