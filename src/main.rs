mod app;
mod error;
mod event;
mod file_loader;
mod tape;
mod ui;

use clap::{Parser, ValueEnum};

use crate::{app::App, error::Result, file_loader::FileLoader};

#[derive(Debug, Parser, ValueEnum, Clone, Copy)]
enum ImageFormat {
    RGB8,
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

fn calculate_frame_size(format: ImageFormat, width: u16, height: u16) -> usize {
    let pixel_count = width as usize * height as usize;

    match format {
        ImageFormat::RGB8 => pixel_count * 3,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    console_subscriber::init();

    let args = Args::parse();

    let frame_size = calculate_frame_size(args.format, args.width, args.height);
    let file_loader = FileLoader::new(&args.path, frame_size, true)?;

    color_eyre::install()?;

    let terminal = ratatui::init();

    App::new(file_loader)?.start(terminal).await?;

    ratatui::restore();

    Ok(())
}
