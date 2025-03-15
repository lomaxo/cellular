use std::thread;
use std::time::Duration;

mod cellular;
use std::io;

use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::prelude::Stylize;

fn main() -> io::Result<()> {

    let mut terminal = ratatui::init();

    let mut app = App {exit: false, grid: cellular::Grid::new(150, 100) };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}



pub struct App {
    exit: bool,
    grid: cellular::Grid,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.grid.randomise_grid();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if crossterm::event::poll(Duration::from_millis(10))? {
                match crossterm::event::read()? {
                    crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                    _ => {}
                }
            }
            self.grid.update_generation();

        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            if key_event.code == KeyCode::Char('q') {
                self.exit = true;
            } else if key_event.code == KeyCode::Char('r') {
                self.grid.randomise_grid();
            } 
            // else if key_event.code == KeyCode::Char('n') {
            //     self.grid.update_generation();
            // }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    // where
        // Self: Sized 
        {
            let vertical_layout = Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)]);
            let [main_area, bottom_area] = vertical_layout.areas(area);
            
            let top_block= Block::bordered()
            .title(Line::from(" Game of Life "))
            .border_set(border::THICK);
            
            let bottom_block= Block::bordered()
                // .title(Line::from(""))
                .border_set(border::THICK);

            Paragraph::new("Press <Q> to quit.\nPress <R> to randomise grid.")
                .bold()
                .block(bottom_block)
                .render(bottom_area, buf);

            let grid_text = self.grid.get_grid_strings().join("\n");
            Paragraph::new(grid_text)
                .block(top_block)
                .render(main_area, buf);
        }
}


// fn main() {
//     println!("Hello, world!");
//     let mut grid = cellular::Grid::new(100,20);
//     grid.randomise_grid();
//     for i in 0..500 {
//         println!("Generation: {i}");
//         grid.update_generation();
//         grid.display();

//         // Create a Duration from the milliseconds
//         let duration = Duration::from_millis(50);
//         // Pause the current thread for the specified duration
//         thread::sleep(duration);
        
//     }
    
// }
