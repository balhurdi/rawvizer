use crossterm::event::{KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

use crate::{
    error::Result,
    event::{AppEvent, EventHandler},
    file_loader::FileLoader,
    ui::VideoPlayerState,
};

pub struct App {
    events: EventHandler,
    file_loader: FileLoader,
    running: bool,
    video_player_state: VideoPlayerState,
}

impl App {
    pub fn new(file_loader: FileLoader) -> Result<Self> {
        Ok(Self {
            events: EventHandler::new(),
            file_loader,
            running: true,
            video_player_state: VideoPlayerState::new(),
        })
    }

    pub async fn start(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
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

    pub(crate) fn video_player_state(&mut self) -> &mut VideoPlayerState {
        &mut self.video_player_state
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            _ => {}
        }
    }
}
