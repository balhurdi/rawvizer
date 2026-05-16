use ratatui::{
    DefaultTerminal,
    layout::Alignment,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{error::Result, file_loader::FileLoader};

pub struct App {
    file_loader: FileLoader,
    running: bool,
}

impl App {
    pub fn new(file_loader: FileLoader) -> Result<Self> {
        Ok(Self {
            file_loader,
            running: true,
        })
    }

    pub fn start(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }

        Ok(())
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
