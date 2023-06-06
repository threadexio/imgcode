use super::command_prelude::*;

use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Png,
    Jpg,
    Gif,
    Ico,
    Bmp,
    OpenExr,
    Tiff,
}

impl From<OutputFormat> for image::ImageOutputFormat {
    fn from(value: OutputFormat) -> Self {
        use OutputFormat::*;
        match value {
            Png => Self::Png,
            Jpg => Self::Jpeg(100),
            Gif => Self::Gif,
            Ico => Self::Ico,
            Bmp => Self::Bmp,
            OpenExr => Self::OpenExr,
            Tiff => Self::Tiff,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum PixelFormat {
    Rgb8,
    Rgba8,
    Rgb32,
    Rgba32,
}

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(help = "Path to input file")]
    input_file: PathBuf,

    #[clap(help = "Path to output file")]
    output_file: PathBuf,

    #[clap(
        short = 'r',
        long = "ratio",
        help = "Aspect ratio of the output image",
        default_value = "1.0"
    )]
    aspect_ratio: f64,

    #[clap(
        short = 'f',
        long = "format",
        help = "Format of the output image",
        default_value = "png"
    )]
    format: OutputFormat,

    #[clap(
        short = 'p',
        long = "pixel",
        help = "Format of the pixels in the image",
        default_value = "rgb8"
    )]
    pixel_format: PixelFormat,
}

pub fn command(_global_args: &CliArgs, args: &Args) -> Result<()> {
    let mut input = util::open_buffered_read(File::options().read(true), &args.input_file)
        .with_context(|| format!("unable to open input `{}`", args.input_file.display()))?;

    let mut output = util::open_buffered_write(File::options().write(true), &args.output_file)
        .with_context(|| format!("unable to open input `{}`", args.output_file.display()))?;

    let mut data = Vec::with_capacity(2048);
    input
        .read_to_end(&mut data)
        .context("unable to read from input")?;

    match args.pixel_format {
        PixelFormat::Rgb8 => imgcode::to_image::<image::RgbImage>(&data, args.aspect_ratio)
            .write_to(&mut output, args.format)?,
        PixelFormat::Rgba8 => imgcode::to_image::<image::RgbaImage>(&data, args.aspect_ratio)
            .write_to(&mut output, args.format)?,
        PixelFormat::Rgb32 => imgcode::to_image::<image::Rgb32FImage>(&data, args.aspect_ratio)
            .write_to(&mut output, args.format)?,
        PixelFormat::Rgba32 => imgcode::to_image::<image::Rgba32FImage>(&data, args.aspect_ratio)
            .write_to(&mut output, args.format)?,
    }

    Ok(())
}
