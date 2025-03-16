use std::time::Duration;

mod cellular;
use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::symbols::border;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::prelude::Stylize;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Widget, Bar, BarChart, BarGroup},
    text::{Text, Line},
};

fn main() -> io::Result<()> {

    let mut terminal = ratatui::init();

    let mut app = App {
        exit: false, 
        grid: cellular::Grid::new(150, 100), 
        running: true, 
        show_stats: false,
    };

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}



pub struct App {
    exit: bool,
    grid: cellular::Grid,
    running: bool,
    show_stats: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let size = terminal.size().unwrap();
        self.grid.resize_grid(size.width as usize, size.height as usize);

        self.grid.randomise_grid();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if crossterm::event::poll(Duration::from_millis(10))? {
                match crossterm::event::read()? {
                    Event::Resize(width, height) => self.grid.resize_grid(width as usize, height as usize),
                    crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                    _ => {}
                }
            }
            if self.running {
                self.grid.update_generation();
            }

        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('r') => self.grid.randomise_grid(),
                KeyCode::Char(' ') => {
                    if self.running { self.running = !self.running; }
                    else { self.grid.update_generation(); }
                }
                KeyCode::Char('s') => self.show_stats = !self.show_stats,
                KeyCode::Char('c') => self.running = true,
                _ => {}
            } 
            
        }
        Ok(())
    }

    fn render_bottom(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Q: quit | R: randomise grid | <space>: pause\\step | C: continue")
        .bold()
        .centered()
        .render(area, buf);
    }

    fn render_grid(&self, area: Rect, buf: &mut Buffer) {
        let top_block= Block::bordered()
        .title(Line::from(" Game of Life "))
        .border_set(border::THICK);

        let mut grid_text = Text::from("");
        let cells = self.grid.get_cells();
        let prev_cells = self.grid.get_prev_cells();

        for row in 0..cells.len() {
            let mut line = Line::from("");
            for col in 0..cells[row].len() {
                if cells[row][col] {
                    // Alive
                    if prev_cells[row][col] {
                        line.spans.push(Span::styled("█", Style::default().fg(Color::Green)));
                    } else {                            
                        line.spans.push(Span::styled("█", Style::default().fg(Color::LightGreen)));
                    }
                } else {
                    // Dead
                    if prev_cells[row][col] {
                        line.spans.push(Span::styled("█", Style::default().fg(Color::DarkGray)));
                    } else {
                        line.spans.push(Span::raw(" "));
                    }
                }
            }
            grid_text.lines.push(line);
        }

        Paragraph::new(grid_text)
            .block(top_block)
            .render(area, buf);
        
    }

    fn render_stats(&self, area: Rect, buf: &mut Buffer) {
        let stats = self.grid.get_stats();
        // let stat_text = Text::from(vec![
        //     Line::from(format!("Births: {}", stats.get_births())), 
        //     Line::from(format!("Survivors: {}", stats.get_survivors())),
        //     Line::from(format!("Deaths: {}", stats.get_deaths())),
        //     Line::from(format!("Population: {}", stats.get_population()))

        //     ]);
        // Paragraph::new(stat_text)
        // .block(
        //     Block::bordered()
        //         .title(Line::from(" Statistics "))
        //         .border_set(border::THICK)
        //     )
        // .render(area, buf); 


    
        let barchart = BarChart::default()
            .block(Block::bordered().title("BarChart"))
            .bar_width(5)
            .bar_gap(1)
            .bar_style(Style::new().yellow())
            .value_style(Style::new().red().bold())
            .label_style(Style::new().white())
            .data(&[("Births", stats.get_births()), ("Survives", stats.get_survivors()), ("Deaths", stats.get_deaths())])
            // .data(BarGroup::default().bars(&[Bar::default().value(10), Bar::default().value(20)]))
            .max(self.grid.get_max_cells()/2);
            //.render(stats_area, buf);
            barchart.render(area, buf);    
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    // where
        // Self: Sized 
        {
            let vertical_layout = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]);
            let [top_area, bottom_area] = vertical_layout.areas(area);
            let horizonal_layout = Layout::horizontal([Constraint::Min(1), Constraint::Length(20)]);
            let [main_area, stats_area];
            
            if self.show_stats {
                [main_area, stats_area] = horizonal_layout.areas(top_area); 
                self.render_stats(stats_area, buf);              
            }
            else {
                main_area = top_area;
            }

            self.render_grid(main_area, buf);
            self.render_bottom(bottom_area, buf);

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
