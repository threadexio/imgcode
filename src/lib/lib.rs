#![warn(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::style
)]

use std::io::prelude::*;

mod private {
    pub trait Sealed {}
}

mod cursor;
mod error;
mod file;
mod traits;

pub use error::{Error, Result};
use traits::Image;

use crate::cursor::ImageCursor;

/// Get the minimum dimensions for an image of type `I` that fits `data`.
///
/// # Safety
///
/// `aspect_ratio` must be greater than (`>`) 0
#[must_use = "unused image dimensions"]
pub fn image_dimensions<I>(data: impl AsRef<[u8]>, aspect_ratio: f64) -> (u32, u32)
where
    I: Image,
{
    let data = data.as_ref();

    if data.is_empty() {
        (0, 0)
    } else {
        let total_bytes = (file::Header::SIZE as u64) + (data.len() as u64);
        let pixel_num = total_bytes / u64::from(I::PIXEL_SIZE);

        min_dimensions_from_pixels(pixel_num, aspect_ratio)
    }
}

/// Get the maximum amount of bytes an image of type `I` with dimensions `X`x`Y` can hold.
#[must_use]
pub fn image_capacity<I>(x: u32, y: u32) -> u64
where
    I: Image,
{
    let pixel_num = u64::from(x) * u64::from(y);
    pixel_num * u64::from(I::PIXEL_SIZE)
}

/// Write `data` to an image with dimensions from [`image_dimensions()`] and return it.
///
/// # Panics
///
/// See [`image_dimensions`]
pub fn to_image<I>(data: impl AsRef<[u8]>, aspect_ratio: f64) -> I
where
    I: Image,
{
    let data = data.as_ref();

    let (image_x, image_y) = image_dimensions::<I>(data, aspect_ratio);

    let mut image = ImageCursor::new(I::new_with_dimensions(image_x, image_y));

    let header = file::Header {
        size: data.len() as u64,
    };

    header.write_to(&mut image).expect("write to image failed");
    image.write_all(data).expect("write to image failed");

    image.into_image()
}

/// Read an image of type `I` and return the contained data in it.
///
/// # Errors
///
/// - Image data size is too large
pub fn from_image<I>(image: I) -> Result<Vec<u8>>
where
    I: Image,
{
    let mut image = ImageCursor::new(image);

    let header = file::Header::read_from(&mut image)?;

    let size: usize = header.size.try_into().map_err(|_| Error::SizeLimit)?;
    let mut data = vec![0u8; size];
    image.read_exact(&mut data)?;

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
/// ```ignore
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
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
fn min_dimensions_from_pixels(pixel_num: u64, aspect_ratio: f64) -> (u32, u32) {
    let pixel_num = pixel_num as f64;

    assert!(
        pixel_num.is_finite() && pixel_num > 0.0,
        "number of pixels must be non-zero"
    );

    assert!(
        aspect_ratio.is_finite() && aspect_ratio > 0.0,
        "aspect ratio must be positive and non-zero"
    );

    // Round up to the nearest integer, we cannot have
    // half-pixels.
    let x = (pixel_num * aspect_ratio).sqrt().ceil() as u32;
    let y = (pixel_num / aspect_ratio).sqrt().ceil() as u32;

    (x, y)
}
