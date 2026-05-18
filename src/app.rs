use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind};
use image::DynamicImage;
use ratatui::DefaultTerminal;

use crate::{
    error::Result,
    event::{AppEvent, Event, EventHandler},
    file_loader::FileLoader,
    tape::{Tape, TapeController},
    ui::VideoPlayerState,
};

pub struct App {
    events: EventHandler,
    running: bool,
    video_player_state: VideoPlayerState,
    tape_controller: TapeController,
    frame_request_in_flight: bool,
}

impl App {
    pub fn new(file_loader: FileLoader) -> Result<Self> {
        let (tape_controller, tape_recv) = Tape::new(file_loader).start();

        Ok(Self {
            events: EventHandler::new(tape_recv),
            running: true,
            video_player_state: VideoPlayerState::new(),
            tape_controller,
            frame_request_in_flight: false,
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
                    AppEvent::NextFrame => self.request_next_frame()?,
                    AppEvent::PreviousFrame => {}
                    AppEvent::FrameReady(image) => self.present_next_frame(image),
                    AppEvent::InternalError(err) => {
                        panic!("{err}")
                    }
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

    fn request_next_frame(&mut self) -> Result<()> {
        if self.frame_request_in_flight {
            return Ok(());
        }
        self.frame_request_in_flight = true;
        self.tape_controller
            .send_event(crate::tape::TapeEvent::NextFrame)?;

        Ok(())
    }

    fn present_next_frame(&mut self, image: DynamicImage) {
        self.frame_request_in_flight = false;
        self.video_player_state.update_picture(image);
    }
}
