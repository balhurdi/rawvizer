mod app;
mod error;
mod event_systems;
mod file_loader;
mod ui;
mod video;

use clap::{Parser, ValueEnum};
use tracing_chrome::{ChromeLayerBuilder, FlushGuard};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    #[arg(long)]
    tracing_file: Option<String>,
}

fn build_frame_format(args: &Args) -> VideoFrameFormat {
    VideoFrameFormat {
        pixel_format: args.format.into(),
        width: args.width,
        height: args.height,
    }
}

#[derive(Default)]
struct TracingContext {
    flush_guard: Option<FlushGuard>,
}

impl TracingContext {
    fn enable_tracing(&mut self, path: &str) {
        let (chrome_layer, guard) = ChromeLayerBuilder::new().file(path).build();

        let console_layer = console_subscriber::ConsoleLayer::builder().spawn();

        tracing_subscriber::registry()
            .with(chrome_layer)
            .with(console_layer)
            .init();

        self.flush_guard = Some(guard)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut tracing_ctx = TracingContext::default();
    let _ = if let Some(tracing_file) = &args.tracing_file {
        tracing_ctx.enable_tracing(&tracing_file)
    };
    let frame_format = build_frame_format(&args);
    let file_loader = FileLoader::new(&args.path, frame_format.frame_size(), true)?;

    color_eyre::install()?;

    let terminal = ratatui::init();

    App::new(file_loader, frame_format)?.start(terminal).await?;

    ratatui::restore();

    Ok(())
}
