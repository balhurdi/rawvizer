use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Clear, Paragraph, StatefulWidget, Widget},
};

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
            CurrentPopUp::Info => todo!(),
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
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: area.height / 3,
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
