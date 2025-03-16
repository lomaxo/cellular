use std::time::Duration;

mod cellular;
use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    symbols::border,
    DefaultTerminal,
    Frame,
    prelude::*,
    widgets::{Block, Paragraph, Widget, BarChart, Sparkline},
    text::{Text, Line},
};

const STATS_WIDTH: u16 = 30;


fn main() -> io::Result<()> {

    let mut terminal = ratatui::init();

    let mut app = App {
        exit: false, 
        grid: cellular::Grid::new(150, 100), 
        running: true, 
        show_stats: false,
        anim_delay: 40,
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
    anim_delay: u64,

}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let size = terminal.size().unwrap();
        self.grid.resize_grid(size.width as usize, size.height as usize);

        self.grid.randomise_grid();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if crossterm::event::poll(Duration::from_millis(self.anim_delay))? {
                match crossterm::event::read()? {
                    Event::Resize(width, height) => self.grid.resize_grid(width as usize, height as usize),
                    crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                    _ => {}
                }
            } else {
                if self.running {
                    self.grid.update_generation();
                }
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
                KeyCode::Left => self.anim_delay = std::cmp::min(self.anim_delay * 2, 1000),
                KeyCode::Right => self.anim_delay = std::cmp::max(self.anim_delay / 2, 1),

                _ => {}
            } 
            
        }
        Ok(())
    }

    fn render_bottom(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Q: quit | R: randomise grid | <space>: pause\\step | C: continue | S: show\\hide stats | ←\\→: change speed")
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
        const SPARK_HEIGHT: u16 = 20;
        const BAR_HEIGHT: u16 = 20;
        const STAT_BORDER_TYPE: ratatui::symbols::border::Set = border::ROUNDED;

        let stats = self.grid.get_stats();
        let [history_area, chart_area,  text_area] = 
            Layout::vertical(
                [Constraint::Max(BAR_HEIGHT),
                Constraint::Max(SPARK_HEIGHT), 
                Constraint::Length(8)]
            ).areas(area);
        let speed = (1000 as f64 / self.anim_delay as f64).log2();

        let stat_text = Text::from(vec![
            Line::from(format!("Births: {}", stats.get_births())), 
            Line::from(format!("Survivors: {}", stats.get_survivors())),
            Line::from(format!("Deaths: {}", stats.get_deaths())),
            Line::from(format!("Population: {}", stats.get_population())),
            Line::from(format!("Age: {}", self.grid.get_age())),
            Line::from(format!("Speed: {:.0}", speed))


        ]);
        Paragraph::new(stat_text)
            .block(
                Block::bordered()
                    .title(Line::from(" Statistics "))
                    .border_set(STAT_BORDER_TYPE)
                )
            .render(text_area, buf); 

        let bar_width = (STATS_WIDTH - 4) / 3;
        BarChart::default()
            .block(Block::bordered().border_set(STAT_BORDER_TYPE))//.title("BarChart"))
            .bar_width(bar_width)
            .bar_gap(1)
            .bar_style(Style::new().yellow())
            .value_style(Style::new().yellow())
            .label_style(Style::new().white())
            .data(&[("Births", stats.get_births()), ("Survives", stats.get_survivors()), ("Deaths", stats.get_deaths())])
            .max(self.grid.get_max_cells()/2)
            .render(chart_area, buf);
        
        
        let history = self.grid.get_history_data();
        let step = std::cmp::max(history.len() / STATS_WIDTH as usize, 1);
        let data: Vec<&u64> = history.iter().step_by(step).collect();  
        // let max_data = self.grid.get_max_cells()/2;  
        let max_data = history.iter().max().copied().unwrap_or(1);
        Sparkline::default()
            .block(Block::bordered().title("Population History").border_set(STAT_BORDER_TYPE))
            .data(data)
            .max(max_data)
            .style(Style::default().red())
            .absent_value_style(Style::default().fg(Color::Red))
            .absent_value_symbol(symbols::shade::FULL)
            .render(history_area, buf);  
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    // where
        // Self: Sized 
        {
            let vertical_layout = Layout::vertical([Constraint::Min(10), Constraint::Length(1)]);
            let [top_area, bottom_area] = vertical_layout.areas(area);
            let horizonal_layout = Layout::horizontal([Constraint::Min(1), Constraint::Length(STATS_WIDTH)]);
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