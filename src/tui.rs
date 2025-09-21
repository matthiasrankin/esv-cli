use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};


#[derive(Debug, Default)]
pub struct App {
    passages: Vec<String>,
    exit: bool,
    scroll: u16
}


impl App {

    pub fn new(passages: Vec<String>) -> Self {
        Self {
            passages,
            exit: false,
            scroll: 0
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('Q') => self.exit(),
            KeyCode::Down => self.scroll_down(),
            KeyCode::Up => self.scroll_up(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn scroll_down(&mut self) {
        self.scroll += 1;
    }

    fn scroll_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_text = self.passages[0].as_str().split("\n").collect::<Vec<&str>>()[0];
        let passage_text = self.passages[0].as_str().split("\n").collect::<Vec<&str>>()[1..].join("\n");

        let title = Line::from(title_text.bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions);

        let paragraph_text = Text::from(passage_text);

        Paragraph::new(paragraph_text)
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}