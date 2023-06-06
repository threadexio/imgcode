use super::command_prelude::*;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(help = "Path to input file")]
    input_file: PathBuf,

    #[clap(help = "Path to output file")]
    output_file: PathBuf,
}

pub fn command(_global_args: &CliArgs, args: &Args) -> Result<()> {
    let mut input = util::open_buffered_read(File::options().read(true), &args.input_file)
        .with_context(|| format!("unable to open input `{}`", args.input_file.display()))?;

    let mut output = util::open_buffered_write(File::options().write(true), &args.output_file)
        .with_context(|| format!("unable to open output `{}`", args.output_file.display()))?;

    let i = image::io::Reader::new(&mut input)
        .with_guessed_format()?
        .decode()?;

    use image::DynamicImage;
    let data = match i {
        DynamicImage::ImageRgb8(v) => imgcode::from_image(v),
        DynamicImage::ImageRgba8(v) => imgcode::from_image(v),
        DynamicImage::ImageRgb32F(v) => imgcode::from_image(v),
        DynamicImage::ImageRgba32F(v) => imgcode::from_image(v),
        _ => bail!("unsupported image pixel format"),
    }?;

    output.write_all(&data)?;

    Ok(())
}
