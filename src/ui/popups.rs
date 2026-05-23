use ratatui::{
    layout::{Alignment, Margin, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

#[derive(Debug, Clone, Copy, Default)]
enum CurrentPopUp {
    Help,
    Info,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, Default)]
struct PopUpState {
    show_help_popup: bool,
    show_info_popup: bool,
}

impl PopUpState {
    fn show_help(&mut self) {
        self.show_help_popup = true;
    }

    fn show_info(&mut self) {
        self.show_info_popup = true;
    }

    fn exit(&mut self) {
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

pub struct PopUp {
    state: PopUpState,
}

impl PopUp {
    pub fn new() -> Self {
        PopUp {
            state: PopUpState::default(),
        }
    }
}

impl Widget for PopUp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        HelpPopUp::new().render(area, buf);
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
[i]            Toggle stream info";

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
