mod app;
mod error;
mod event_systems;
mod file_loader;
mod ui;
mod video;

use clap::{Parser, ValueEnum};

use crate::{
    app::App,
    error::Result,
    file_loader::FileLoader,
    video::{PixelFormat, VideoFrameFormat},
};

#[derive(Debug, Parser, ValueEnum, Clone, Copy)]
enum ImageFormat {
    RGB8,
    V210,
}

impl From<ImageFormat> for PixelFormat {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::RGB8 => PixelFormat::RGB8,
            ImageFormat::V210 => PixelFormat::V210,
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(long)]
    width: u16,
    #[arg(long)]
    height: u16,
    #[arg(long)]
    format: ImageFormat,
}

fn build_frame_format(args: &Args) -> VideoFrameFormat {
    VideoFrameFormat {
        pixel_format: args.format.into(),
        width: args.width,
        height: args.height,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    console_subscriber::init();

    let args = Args::parse();

    let frame_format = build_frame_format(&args);
    let file_loader = FileLoader::new(&args.path, frame_format.frame_size(), true)?;

    color_eyre::install()?;

    let terminal = ratatui::init();

    App::new(file_loader, frame_format)?.start(terminal).await?;

    ratatui::restore();

    Ok(())
}
