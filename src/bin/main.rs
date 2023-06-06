use anyhow::{bail, Result};
use clap::Parser;

use std::fs::File;
use std::io;
use std::path::PathBuf;

use io::prelude::*;
use io::{BufReader, BufWriter};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Png,
    Jpg,
    Gif,
    Ico,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum PixelFormat {
    Rgb8,
    Rgba8,
}

impl From<OutputFormat> for image::ImageOutputFormat {
    fn from(value: OutputFormat) -> Self {
        use OutputFormat::*;
        match value {
            Png => Self::Png,
            Jpg => Self::Jpeg(100),
            Gif => Self::Gif,
            Ico => Self::Ico,
        }
    }
}

#[derive(Debug, clap::Args)]
struct EncodeArgs {
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

#[derive(Debug, clap::Args)]
struct DecodeArgs {}

#[derive(Debug, clap::Subcommand)]
enum CliCommands {
    Decode(DecodeArgs),
    Encode(EncodeArgs),
}

#[derive(Debug, Parser)]
struct CliArgs {
    #[clap(short = 'i', long = "input", help = "Path to input file")]
    input_file: PathBuf,

    #[clap(short = 'o', long = "output", help = "Path to output file")]
    output_file: PathBuf,

    #[clap(subcommand)]
    command: CliCommands,
}

fn decode(global_args: &CliArgs, _: &DecodeArgs) -> Result<()> {
    let input_file = File::options().read(true).open(&global_args.input_file)?;
    let mut input_file = BufReader::new(input_file);

    let output_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&global_args.output_file)?;
    let mut output_file = BufWriter::new(output_file);

    let i = image::io::Reader::new(&mut input_file)
        .with_guessed_format()?
        .decode()?;

    use image::DynamicImage;
    let data = match i {
        DynamicImage::ImageRgb8(v) => imgcode::from_image(v),
        DynamicImage::ImageRgba8(v) => imgcode::from_image(v),
        _ => bail!("unsupported image pixel format"),
    }?;

    output_file.write_all(&data)?;

    Ok(())
}

fn encode(global_args: &CliArgs, cmd_args: &EncodeArgs) -> Result<()> {
    let mut data = Vec::with_capacity(2048);

    let input_file = File::options().read(true).open(&global_args.input_file)?;
    let mut input_file = BufReader::new(input_file);

    let output_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&global_args.output_file)?;
    let mut output_file = BufWriter::new(output_file);

    input_file.read_to_end(&mut data)?;

    match cmd_args.pixel_format {
        PixelFormat::Rgb8 => imgcode::to_image::<image::RgbImage>(&data, cmd_args.aspect_ratio)
            .write_to(&mut output_file, cmd_args.format)?,
        PixelFormat::Rgba8 => imgcode::to_image::<image::RgbaImage>(&data, cmd_args.aspect_ratio)
            .write_to(&mut output_file, cmd_args.format)?,
    }

    Ok(())
}

fn try_main() -> Result<()> {
    let global_args = CliArgs::parse();

    match &global_args.command {
        CliCommands::Decode(cmd_args) => decode(&global_args, cmd_args),
        CliCommands::Encode(cmd_args) => encode(&global_args, cmd_args),
    }
}

fn main() {
    use std::process::exit;

    if let Err(e) = try_main() {
        eprintln!("{e:#?}");
    }

    exit(-1);
}
