use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind};
use image::{DynamicImage, RgbImage};
use ratatui::DefaultTerminal;

use crate::{
    error::{Error, Result},
    event::{AppEvent, Event, EventHandler},
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
                Event::Crossterm(ev) => match ev {
                    CrosstermEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(&key_event)
                    }
                    _ => {}
                },

                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.running = false,
                    AppEvent::NextFrame => self.update_to_next_frame()?,
                    AppEvent::PreviousFrame => {}
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
            KeyCode::Right => self.events.send(AppEvent::NextFrame),
            KeyCode::Left => self.events.send(AppEvent::PreviousFrame),
            _ => {}
        }
    }

    fn update_to_next_frame(&mut self) -> Result<()> {
        if let Some(Ok(fb)) = self.file_loader.next() {
            let image = RgbImage::from_raw(1920, 1080, fb.data().to_vec())
                .ok_or(Error::InvalidBufferSize)?;
            let dynamic_image = DynamicImage::from(image);
            self.video_player_state.update_picture(dynamic_image);
        }

        Ok(())
    }
}
