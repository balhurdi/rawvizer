use std::fmt::Debug;

use image::DynamicImage;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Size},
    style::Style,
    widgets::{Paragraph, StatefulWidget, Widget},
};
use ratatui_image::{Resize, StatefulImage, picker::Picker, protocol::StatefulProtocol};

pub struct VideoPlayerState {
    image: Option<StatefulProtocol>,
    picker: Picker,
}

impl Debug for VideoPlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.picker)
    }
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

#[derive(Debug)]
pub struct VideoPlayer;

impl VideoPlayer {
    pub fn new() -> Self {
        Self
    }
}

impl StatefulWidget for VideoPlayer {
    type State = VideoPlayerState;

    #[tracing::instrument]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let Some(img) = &mut state.image {
            let resize = Resize::Scale(None);
            let fitting = img.size_for(resize.clone(), Size::new(area.width, area.height));

            let x_off = (area.width.saturating_sub(fitting.width)) / 2;
            let y_off = (area.height.saturating_sub(fitting.height)) / 2;

            let img_area = Rect::new(
                area.x + x_off,
                area.y + y_off,
                fitting.width,
                fitting.height,
            );

            StatefulImage::default()
                .resize(resize)
                .render(img_area, buf, img);
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
