use std::fs::File;
use std::io;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

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
}

#[derive(Debug, clap::Subcommand)]
enum CliCommands {
    Decode,
    Encode,
}

#[derive(Debug, Parser)]
struct CliArgs {
    #[clap(short = 'f', long = "file")]
    img_file: PathBuf,

    #[clap(subcommand)]
    command: CliCommands,
}

fn decode(args: &CliArgs) -> Result<()> {
    let file = File::options().read(true).open(&args.img_file)?;
    let reader = io::BufReader::new(file);

    let img = image::io::Reader::new(reader)
        .with_guessed_format()?
        .decode()?;

    decode_image(&img, io::stdout())?;

    Ok(())
}

fn encode(args: &CliArgs) -> Result<()> {
    // read data from stdin
    let reader = io::BufReader::new(io::stdin());

    let mut img = DynamicImage::ImageRgba8(image::RgbaImage::new(64, 64));
    encode_bytes_to_image(&mut img, reader)?;

    let out_file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&args.img_file)?;
    let mut writer = io::BufWriter::new(out_file);
    img.write_to(&mut writer, image::ImageFormat::Png)?;

    Ok(())
}

fn try_main() -> Result<()> {
    let args = CliArgs::parse();

    match args.command {
        CliCommands::Encode => encode(&args),
        CliCommands::Decode => decode(&args),
    }
}

fn main() {
    use std::process::exit;

    if let Err(e) = try_main() {
        eprintln!("{e:#?}");
    }

    exit(-1);
}
