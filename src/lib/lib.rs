use std::io;

use io::{Read, Write};

mod error;
mod private;
mod traits;
mod util;

pub use error::Error;
use error::*;

mod reader;
use reader::*;

mod writer;
use writer::*;

#[derive(Debug)]
pub struct Header {
    pub size: u64,
}

impl Header {
    const HEADER_SIZE: usize = 8;

    pub fn write_to<W>(&self, mut w: W) -> io::Result<()>
    where
        W: Write,
    {
        w.write_all(&self.size.to_be_bytes())?;

        Ok(())
    }

    pub fn read_from<R>(mut r: R) -> io::Result<Self>
    where
        R: Read,
    {
        let mut raw: [u8; 8] = [0; 8];
        let _ = r.read(&mut raw)?;

        Ok(Self {
            size: u64::from_be_bytes(raw),
        })
    }
}

pub fn to_image<I>(data: impl AsRef<[u8]>, aspect_ratio: f64) -> Result<I>
where
    I: traits::Image,
{
    let data = data.as_ref();

    let total_bytes: u64 = usize::checked_add(Header::HEADER_SIZE, data.len())
        .ok_or(Error::SizeLimit)?
        .try_into()
        .map_err(|_| Error::SizeLimit)?;

    let total_pixels: u64 = ((total_bytes as f64) / (I::PIXEL_SIZE as f64)).ceil() as u64;
    let (image_x, image_y) = min_dimensions_from_pixels(total_pixels, aspect_ratio);

    let mut w = ImageWriter::new(I::new_with_dimensions(image_x, image_y));

    let header = Header {
        size: data.len() as u64,
    };

    header.write_to(&mut w)?;
    w.write_all(data)?;

    Ok(w.into_image())
}

pub fn from_image<I>(image: I) -> Result<Vec<u8>>
where
    I: traits::Image,
{
    let mut r = ImageReader::new(image);

    let header = Header::read_from(&mut r)?;

    let data_length: usize = header.size.try_into().map_err(|_| Error::SizeLimit)?;

    let mut data = vec![0u8; data_length];
    r.read_exact(&mut data)?;

    Ok(data)
}

/// Find the minimum dimensions of an image
/// given the total number of pixels in it
/// and the aspect ratio.
///
/// # Safety
///
/// This function will panic if any of the
/// following are true:
///
/// 1. `pixel_num == 0`
/// 2. `aspect_ratio <= 0`
///
/// # How?
///
/// Consider the following variables:
/// ```ignore
/// c = pixel_num
/// λ = aspect_ratio
/// ```
///
/// And `m`, `n` the number of pixels
/// on the X and Y axis respectively.
///
/// ```ignore
/// |----- m -----|
/// +-------------+ -
/// |             | |
/// |             | n
/// |             | |
/// +-------------+ -
/// ```
///
/// The following holds true then:
///
/// ```ignore
/// c > 0
/// λ > 0
/// m, n ∈ Z*
/// ```
///
/// From all the above we can derive the following:
///
/// ```ignore
/// (1) m * n = c
/// ```
///
/// The product of `m` and `n` must be equal to the
/// number of pixels in the image.
///
/// ```no_run
/// (2) m / n = λ
/// ```
///
/// `m` over `n` must be equal to the given aspect ratio.
///
/// From eq. (2) we have:
///
/// ```ignore
///                              λ != 0
/// (2) m / n = λ <=> m = n * λ <======> n = m / λ
/// ```
///
/// Replacing `n` from above in eq. (1) we have:
///
/// ```ignore
/// (1) m * n = c <=> m * (m / λ) = c
///               <=> m^2 / λ = c
///               <=> m^2 = λ * c
/// ```
///
/// The product of `λ` and `c` is positive and non-zero for
/// any `λ` and `c`, therefore eq. (1) becomes:
///
/// ```ignore
/// (1) m^2 = λ * c <=> m = √(λ * c )
/// ```
///
/// Replacing `m` in eq. (2) and solving for `n`:
///
/// ```ignore
/// (2) n = m / λ <=> n = √(λ * c) / λ
///               <=> n = ( √λ * √c ) / λ
///               <=> n = ( √λ * √c * √λ ) / ( λ * √λ )
///               <=> n = ( (√λ)^2 * √c ) / ( λ * √λ )
///               <=> n = ( λ * √c ) / ( λ * √λ )
///               <=> n = √c / √λ
///               <=> n = √(c / λ )
/// ```
///
/// And we now have the formulas for `m` and `n`.
///
/// ```ignore
/// m = √(c * λ)
/// n = √(c / λ)
/// ```
fn min_dimensions_from_pixels(pixel_num: u64, aspect_ratio: f64) -> (u64, u64) {
    let pixel_num = pixel_num as f64;

    if !pixel_num.is_finite() || pixel_num <= 0.0 {
        panic!("number of pixels must be non-zero")
    }

    if !aspect_ratio.is_finite() || aspect_ratio <= 0.0 {
        panic!("aspect ratio must be positive and non-zero")
    }

    // Round up to the nearest integer, we cannot have
    // half-pixels.
    let x = (pixel_num * aspect_ratio).sqrt().ceil() as u64;
    let y = (pixel_num / aspect_ratio).sqrt().ceil() as u64;

    (x, y)
}
