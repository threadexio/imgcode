pub trait Sealed {}

pub trait ImageFormat: Sealed {
    const BYTES_PER_PIXEL: u32;
}

pub struct Rgb8;
pub struct Rgba8;
pub struct Rgb32;
pub struct Rgba32;

impl Sealed for Rgb8 {}
impl Sealed for Rgba8 {}
impl Sealed for Rgb32 {}
impl Sealed for Rgba32 {}
