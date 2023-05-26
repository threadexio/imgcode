use std::io;

use crate::{traits::Image, util::*};

pub struct ImageReader<I>
where
    I: Image,
{
    image: I,
    pos: u64,
}

impl<I> ImageReader<I>
where
    I: Image,
{
    pub fn new(image: I) -> Self {
        Self { image, pos: 0 }
    }
}

impl<I> io::Read for ImageReader<I>
where
    I: Image,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0;

        loop {
            if bytes_read == buf.len() {
                break;
            }

            let (pixel_x, pixel_y, rem) = xyrem_from_pos::<I>(self.pos, self.image.width());

            let pixel = match self.image.read_pixel(pixel_x, pixel_y) {
                Some(v) => v,
                None => break,
            };

            let rem = rem as usize;
            let i = copy_min_len(&pixel[rem..], &mut buf[bytes_read..]);
            if i == 0 {
                break;
            }

            bytes_read += i;
            self.pos += i as u64;
        }

        Ok(bytes_read)
    }
}
