use ratatui::{
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, BorderType, StatefulWidget, Widget},
};

use crate::{app::App, ui::video_player::VideoPlayer};

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]);
        let [video, controls] = area.layout(&layout);

        let controls_layout = Layout::horizontal([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]);

        let [file_name, playback_status, current_frame] = controls.layout(&controls_layout);

        let video_block = Block::bordered()
            .title("Raw Vis")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let file_name_block = Block::bordered();
        let playback_status_block = Block::bordered();
        let current_frame_block = Block::bordered();

        let video_block_inner = video_block.inner(video);
        video_block.render(video, buf);

        let video_player = VideoPlayer::new();
        video_player.render(video_block_inner, buf, self.video_player_state());

        file_name_block.render(file_name, buf);
        playback_status_block.render(playback_status, buf);
        current_frame_block.render(current_frame, buf);
    }
}
