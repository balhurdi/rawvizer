use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal,
    layout::Alignment,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    error::Result,
    event::{AppEvent, EventHandler},
    file_loader::FileLoader,
};

pub struct App {
    events: EventHandler,
    file_loader: FileLoader,
    running: bool,
}

impl App {
    pub fn new(file_loader: FileLoader) -> Result<Self> {
        Ok(Self {
            events: EventHandler::new(),
            file_loader,
            running: true,
        })
    }

    pub async fn start(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                crate::event::Event::Crossterm(ev) => match ev {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_event(&key_event)
                    }
                    _ => {}
                },

                crate::event::Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.running = false,
                },
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title("{{project-name}}")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                ",
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);
    }
}
