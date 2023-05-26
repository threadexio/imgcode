use std::io;

use crate::{traits::Image, util::*};

pub struct ImageWriter<I> {
    image: I,
    pos: u64,
}

impl<I> ImageWriter<I> {
    pub fn new(image: I) -> Self {
        Self { image, pos: 0 }
    }

    pub fn into_image(self) -> I {
        self.image
    }
}

impl<I> io::Write for ImageWriter<I>
where
    I: Image,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes_written = 0;

        loop {
            if bytes_written == buf.len() {
                break;
            }

            let (pixel_x, pixel_y, rem) = xyrem_from_pos::<I>(self.pos, self.image.width());

            let i = match self
                .image
                .write_pixel(pixel_x, pixel_y, &buf[bytes_written..], rem)
            {
                Some(v) if v != 0 => v,
                _ => break,
            };

            bytes_written += i;
            self.pos += i as u64;
        }

        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
