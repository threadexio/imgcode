use crate::util::*;

pub trait Image {
    const PIXEL_SIZE: u64;

    fn new_with_dimensions(x: u64, y: u64) -> Self
    where
        Self: Sized;

    fn width(&self) -> u64;

    fn write_pixel(&mut self, x: u64, y: u64, buf: &[u8], pos: u64) -> Option<usize>;
    fn read_pixel(&self, x: u64, y: u64) -> Option<&[u8]>;
}

impl<T> Image for &mut T
where
    T: Image,
{
    const PIXEL_SIZE: u64 = T::PIXEL_SIZE;

    fn write_pixel(&mut self, x: u64, y: u64, buf: &[u8], pos: u64) -> Option<usize> {
        (**self).write_pixel(x, y, buf, pos)
    }

    fn read_pixel(&self, x: u64, y: u64) -> Option<&[u8]> {
        (**self).read_pixel(x, y)
    }

    fn width(&self) -> u64 {
        (**self).width()
    }

    fn new_with_dimensions(_: u64, _: u64) -> Self
    where
        Self: Sized,
    {
        panic!()
    }
}

impl Image for image::RgbImage {
    const PIXEL_SIZE: u64 = 3;

    fn write_pixel(&mut self, x: u64, y: u64, buf: &[u8], pos: u64) -> Option<usize> {
        let pos = pos as usize;
        let pixel = self.read_pixel(x, y)?;

        let mut channels = [0u8; 3];
        copy_min_len(pixel, &mut channels[..pos]);

        let i = copy_min_len(buf, &mut channels[pos..]);
        self.put_pixel(x as u32, y as u32, image::Rgb(channels));
        Some(i)
    }

    fn read_pixel(&self, x: u64, y: u64) -> Option<&[u8]> {
        let pixel = self.get_pixel_checked(x as u32, y as u32)?;
        Some(&pixel.0)
    }

    fn width(&self) -> u64 {
        self.width().into()
    }

    fn new_with_dimensions(x: u64, y: u64) -> Self
    where
        Self: Sized,
    {
        Self::new(x as u32, y as u32)
    }
}

impl Image for image::RgbaImage {
    const PIXEL_SIZE: u64 = 4;

    fn write_pixel(&mut self, x: u64, y: u64, buf: &[u8], pos: u64) -> Option<usize> {
        let pos = pos as usize;
        let pixel = self.read_pixel(x, y)?;

        let mut channels = [0u8; 4];
        copy_min_len(pixel, &mut channels[..pos]);

        let i = copy_min_len(buf, &mut channels[pos..]);
        self.put_pixel(x as u32, y as u32, image::Rgba(channels));
        Some(i)
    }

    fn read_pixel(&self, x: u64, y: u64) -> Option<&[u8]> {
        let pixel = self.get_pixel_checked(x as u32, y as u32)?;
        Some(&pixel.0)
    }

    fn width(&self) -> u64 {
        self.width().into()
    }

    fn new_with_dimensions(x: u64, y: u64) -> Self
    where
        Self: Sized,
    {
        Self::new(x as u32, y as u32)
    }
}
