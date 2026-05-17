use image::DynamicImage;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Paragraph, StatefulWidget, Widget},
};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

pub struct VideoPlayerState {
    image: Option<StatefulProtocol>,
    picker: Picker,
}

impl VideoPlayerState {
    pub fn new() -> Self {
        let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks());

        Self {
            image: None,
            picker,
        }
    }

    pub fn update_picture(&mut self, image: DynamicImage) {
        let stateful_image = self.picker.new_resize_protocol(image);
        self.image = Some(stateful_image);
    }
}

pub struct VideoPlayer;

impl VideoPlayer {
    pub fn new() -> Self {
        Self
    }
}

impl StatefulWidget for VideoPlayer {
    type State = VideoPlayerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let Some(img) = &mut state.image {
            StatefulImage::default().render(area, buf, img);
        } else {
            let loading_paragraph = Paragraph::new("Loading...")
                .centered()
                .style(Style::new().bold());

            let layout = Layout::vertical([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]);

            let [_, center, _] = area.layout(&layout);

            loading_paragraph.render(center, buf);
        }
    }
}
