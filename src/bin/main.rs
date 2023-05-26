use anyhow::{bail, Result};
use clap::Parser;

use std::fs::File;
use std::io;
use std::path::PathBuf;

use io::prelude::*;
use io::{BufReader, BufWriter};

/*
use image::{DynamicImage, GenericImage, GenericImageView, Pixels};
use io::{Read, Write};
use serde::{Deserialize, Serialize};

use bincode::Options;
macro_rules! bincode {
    () => {
        bincode::options()
            .reject_trailing_bytes()
            .with_big_endian()
            .with_fixint_encoding()
            .with_no_limit()
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct DataHeader {
    /// Serialized data length
    size: u64,
}

fn encode_bytes_to_image<R>(image: &mut DynamicImage, mut reader: R) -> io::Result<()>
where
    R: Read,
{
    let max_x = image.width();
    let max_y = image.height();

    // header end
    let mut x = 2;
    let mut y = 0;

    let mut bytes_written = 0;

    loop {
        let mut block: [u8; 4] = [0u8; 4];
        let bytes_read = reader.read(&mut block)?;
        if bytes_read == 0 {
            break;
        }

        for _ in 0..bytes_read {
            image.put_pixel(x, y, image::Rgba(block));

            x += 1;
            if x == max_x {
                x = 0;
                y += 1;
            }

            if y == max_y {
                panic!("reached the limit of rows")
            }

            bytes_written += 1;
        }
    }

    let header = DataHeader {
        size: bytes_written,
    };
    let raw_header = bincode!().serialize(&header).unwrap();
    let mut blocks = raw_header.chunks_exact(4);
    image.put_pixel(
        0,
        0,
        image::Rgba(blocks.next().unwrap().try_into().unwrap()),
    );
    image.put_pixel(
        1,
        0,
        image::Rgba(blocks.next().unwrap().try_into().unwrap()),
    );

    Ok(())
}

fn decode_bytes_from_pixels<W>(
    pixels: &mut Pixels<DynamicImage>,
    mut writer: W,
    amt: usize,
) -> io::Result<()>
where
    W: Write,
{
    for (_, _, rgba) in pixels.take(amt / 4) {
        writer.write_all(&rgba.0)?;
    }

    Ok(())
}

fn decode_image<W>(image: &image::DynamicImage, mut writer: W) -> Result<()>
where
    W: Write,
{
    use image::GenericImageView;

    let mut pixels = image.pixels();

    let mut header_buf = io::Cursor::new(vec![0u8; 8]);
    decode_bytes_from_pixels(&mut pixels, &mut header_buf, 8)?;
    let header: DataHeader = bincode!().deserialize(header_buf.get_ref())?;

    let data_length = header.size as usize;
    let remaining_bytes = data_length % 4;

    decode_bytes_from_pixels(&mut pixels, &mut writer, data_length)?;
    if remaining_bytes != 0 {
        let (_, _, rgba) = pixels
            .next()
            .ok_or(io::Error::from(io::ErrorKind::UnexpectedEof))?;

        writer.write_all(&rgba.0)?;
    }

    Ok(())
} */

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Png,
    Jpg,
    Gif,
    Ico,
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
    #[clap(short = 'f', long = "format", help = "Format of the output image")]
    format: OutputFormat,
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
    #[clap(short = 'i', long = "input", help = "Path to input file", global(true))]
    input_file: Option<PathBuf>,

    #[clap(
        short = 'o',
        long = "output",
        help = "Path to output file",
        global(true)
    )]
    output_file: Option<PathBuf>,

    #[clap(subcommand)]
    command: CliCommands,
}

fn decode(global_args: &CliArgs, cmd_args: &DecodeArgs) -> Result<()> {
    let input_file = File::options().read(true).open(
        global_args.input_file.as_ref().unwrap(), //.unwrap_or(&PathBuf::from("/dev/stdin")),
    )?;
    let mut input_file = BufReader::new(input_file);

    let output_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(
            global_args
                .output_file
                .as_ref()
                .unwrap_or(&PathBuf::from("/dev/stdout")),
        )?;
    let mut output_file = BufWriter::new(output_file);

    let i = image::io::Reader::new(&mut input_file)
        .with_guessed_format()?
        .decode()?;

    use image::DynamicImage;
    let data = match i {
        DynamicImage::ImageRgb8(v) => imgcode::from_image(v),
        DynamicImage::ImageRgba8(v) => imgcode::from_image(v),
        _ => bail!("unsupported image format"),
    }?;

    output_file.write_all(&data)?;

    Ok(())
}

fn encode(global_args: &CliArgs, cmd_args: &EncodeArgs) -> Result<()> {
    let mut data = Vec::with_capacity(2048);

    let input_file = File::options().read(true).open(
        global_args
            .input_file
            .as_ref()
            .unwrap_or(&PathBuf::from("/dev/stdin")),
    )?;
    let mut input_file = BufReader::new(input_file);

    let output_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(
            global_args
                .output_file
                .as_ref()
                .unwrap_or(&PathBuf::from("/dev/stdout")),
        )?;
    let mut output_file = BufWriter::new(output_file);

    input_file.read_to_end(&mut data)?;
    let i = imgcode::to_image::<image::RgbaImage>(&data, 1.0)?;
    i.write_to(&mut output_file, cmd_args.format)?;

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
