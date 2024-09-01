use std::{
    fs::File,
    io::{self, stdin, Cursor, Read},
    path::{Path, PathBuf},
};

use clap::Parser;
use image::{
    codecs::avif::AvifEncoder, ExtendedColorType, ImageEncoder, ImageError, ImageReader, RgbaImage,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Image Error: {0}")]
    ImageError(#[from] ImageError),
}

/// Converts an image to avif
#[derive(Parser, Debug)]
struct Args {
    /// Output file
    #[arg(short, long)]
    output: PathBuf,
    /// Goes from 1 (best quality) to 10 (very fast)
    #[arg(short, long)]
    speed: u8,
    /// Goes from 1 which is basically garbage to 100 which is pretty much placebo
    #[arg(short, long)]
    quality: u8,
}

fn main() -> Result<(), Error> {
    let Args {
        speed,
        quality,
        output,
    } = Args::parse();

    let mut image_buffer = Vec::new();
    stdin().lock().read_to_end(&mut image_buffer)?;

    let image = ImageReader::new(Cursor::new(image_buffer))
        .with_guessed_format()?
        .decode()?
        .into_rgba8();

    encode(image, speed, quality, output)?;

    Ok(())
}

fn encode<P: AsRef<Path>>(
    image: RgbaImage,
    speed: u8,
    quality: u8,
    output: P,
) -> Result<(), Error> {
    let encoder = AvifEncoder::new_with_speed_quality(File::create(output)?, speed, quality);

    encoder.write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        ExtendedColorType::Rgba8,
    )?;

    Ok(())
}
