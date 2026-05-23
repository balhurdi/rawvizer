use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Clear, Paragraph, StatefulWidget, Widget},
};

use crate::video::VideoFrameFormat;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum CurrentPopUp {
    Help,
    Info,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PopUpState {
    show_help_popup: bool,
    show_info_popup: bool,
    frame_format: Option<VideoFrameFormat>,
}

impl PopUpState {
    pub fn show_help(&mut self) {
        self.show_help_popup = true;
        self.show_info_popup = false;
    }

    pub fn show_info(&mut self) {
        self.show_info_popup = true;
        self.show_help_popup = false;
    }

    pub fn exit(&mut self) {
        self.show_help_popup = false;
        self.show_info_popup = false;
    }

    pub fn with_frame_format(frame_format: VideoFrameFormat) -> Self {
        Self {
            frame_format: Some(frame_format),
            ..Default::default()
        }
    }

    fn current_pop_up(&self) -> CurrentPopUp {
        if self.show_help_popup {
            CurrentPopUp::Help
        } else if self.show_info_popup {
            CurrentPopUp::Info
        } else {
            CurrentPopUp::None
        }
    }
}

pub struct PopUp;

impl PopUp {
    pub fn new() -> Self {
        PopUp
    }
}

impl StatefulWidget for PopUp {
    type State = PopUpState;
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        match state.current_pop_up() {
            CurrentPopUp::Help => HelpPopUp::new().render(area, buf),
            CurrentPopUp::Info => {
                if let Some(frame_format) = state.frame_format {
                    InfoPopUp::new(frame_format).render(area, buf);
                }
            }
            CurrentPopUp::None => {}
        }
    }
}

struct HelpPopUp;

impl HelpPopUp {
    fn new() -> Self {
        Self
    }
}

impl Widget for HelpPopUp {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let popup = Rect {
            x: area.width.saturating_sub(32),
            y: 1,
            width: 32,
            height: area.height.saturating_sub(2),
        };

        Clear.render(popup, buf);

        let content = "\
[Right Arrow]  Next frame
[Left Arrow]   Previous frame
[Space]        Play / Pause
[h]            Toggle help
[i]            Toggle stream info
[q]            Exit program";

        Block::bordered()
            .title(" Esc to exit ")
            .title(Line::from(" Help ").centered())
            .border_type(BorderType::Rounded)
            .white()
            .on_black()
            .render(popup, buf);

        Paragraph::new(content).alignment(Alignment::Left).render(
            popup.inner(Margin {
                horizontal: 2,
                vertical: 2,
            }),
            buf,
        );
    }
}

struct InfoPopUp {
    video_frame_foramt: VideoFrameFormat,
}

impl InfoPopUp {
    fn new(video_frame_foramt: VideoFrameFormat) -> Self {
        Self { video_frame_foramt }
    }
}

impl Widget for InfoPopUp {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let popup = Rect {
            x: area.width.saturating_sub(32),
            y: 1,
            width: 32,
            height: area.height.saturating_sub(2),
        };

        Clear.render(popup, buf);

        let content = format!(
            "\
Pixel Format: {:?}
Width:        {}
Height:       {}
Frame Size:   {} bytes",
            self.video_frame_foramt.pixel_format,
            self.video_frame_foramt.width,
            self.video_frame_foramt.height,
            self.video_frame_foramt.frame_size(),
        );

        Block::bordered()
            .title(" Esc to exit ")
            .title(Line::from(" Info ").centered())
            .border_type(BorderType::Rounded)
            .white()
            .on_black()
            .render(popup, buf);

        Paragraph::new(content).alignment(Alignment::Left).render(
            popup.inner(Margin {
                horizontal: 2,
                vertical: 2,
            }),
            buf,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initially_none() {
        let state = PopUpState::default();
        assert_eq!(state.current_pop_up(), CurrentPopUp::None);
    }

    #[test]
    fn show_help_does_not_show_info() {
        let mut state = PopUpState::default();
        state.show_help();
        assert_eq!(state.current_pop_up(), CurrentPopUp::Help);
    }

    #[test]
    fn show_info_does_not_show_help() {
        let mut state = PopUpState::default();
        state.show_info();
        assert_eq!(state.current_pop_up(), CurrentPopUp::Info);
    }

    #[test]
    fn show_help_shows_only_help() {
        let mut state = PopUpState::default();
        state.show_info();
        state.show_help();
        assert_eq!(state.current_pop_up(), CurrentPopUp::Help);
    }

    #[test]
    fn show_info_shows_only_info() {
        let mut state = PopUpState::default();
        state.show_help();
        state.show_info();
        assert_eq!(state.current_pop_up(), CurrentPopUp::Info);
    }

    #[test]
    fn exit_clears_both() {
        let mut state = PopUpState::default();
        state.show_help();
        state.show_info();
        state.exit();
        assert_eq!(state.current_pop_up(), CurrentPopUp::None);
    }
}
