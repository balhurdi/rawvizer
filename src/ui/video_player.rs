use std::fmt::Debug;

use image::DynamicImage;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Size},
    style::Style,
    widgets::{Paragraph, StatefulWidget, Widget},
};
use ratatui_image::{Resize, ResizeEncodeRender, StatefulImage, picker::Picker, protocol::StatefulProtocol};

pub struct VideoPlayerState {
    image: Option<StatefulProtocol>,
    picker: Picker,
    last_area: Option<Size>,
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
            last_area: None,
        }
    }

    pub fn update_picture(&mut self, image: DynamicImage) {
        let mut protocol = self.picker.new_resize_protocol(image);
        if let Some(last_area) = self.last_area {
            let resize = Resize::Scale(None);
            let fitting = protocol.size_for(resize.clone(), last_area);
            protocol.resize_encode(&resize, fitting);
        }
        self.image = Some(protocol);
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
        state.last_area = Some(Size::new(area.width, area.height));

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
