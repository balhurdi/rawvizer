mod error;
mod file_loader;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser, ValueEnum, Clone, Copy)]
enum ImageFormat {
    RGBA8,
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

fn main() {
    let args = Args::parse();
    let conf = viuer::Config {
        x: 0,
        y: 0,
        ..Default::default()
    };

    let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));
    viuer::print(&img, &conf).expect("Image printing failed.");
}
