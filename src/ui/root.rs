use ratatui::{
    layout::HorizontalAlignment,
    widgets::{Block, BorderType, StatefulWidget, Widget},
};

use crate::{app::App, ui::video_player::VideoPlayer};

impl Widget for &mut App {
    #[tracing::instrument]
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let video_block = Block::bordered()
            .title_bottom("[ h - Help | i - Info ]")
            .title_alignment(HorizontalAlignment::Right)
            .border_type(BorderType::Rounded);

        let video_block_inner = video_block.inner(area);
        video_block.render(area, buf);

        let video_player = VideoPlayer::new();
        video_player.render(video_block_inner, buf, self.video_player_state());
    }
}
