use std::io::{self, prelude::*};

use crate::traits::Image;

#[allow(clippy::module_name_repetitions)]
pub struct ImageCursor<I> {
    image: I,
    pos: u64,
}

impl<I> ImageCursor<I> {
    pub fn new(image: I) -> Self {
        Self { image, pos: 0 }
    }

    pub fn into_image(self) -> I {
        self.image
    }
}

impl<I> ImageCursor<I>
where
    I: Image,
{
    /// Get the total amount of bytes this image can hold.
    #[must_use]
    pub fn capacity(&self) -> u64 {
        u64::from(self.image.width()) * u64::from(self.image.height()) * u64::from(I::PIXEL_SIZE)
    }

    /// Get the `x` and `y` position of the pixel `pos` is pointing at.
    /// Also returns the offset in that pixel.
    ///
    /// Returns:
    ///
    /// (`x coordinate`, `y coordinate`, `offset inside pixel`)
    #[allow(clippy::cast_possible_truncation)]
    fn xyrem_from_pos(&self, pos: u64) -> (u32, u32, u64) {
        let width = self.image.width() * I::PIXEL_SIZE;

        let y = (pos / u64::from(width)) as u32;

        let y_width = u64::from(y) * u64::from(width);

        let x = ((pos - y_width) / u64::from(I::PIXEL_SIZE)) as u32;
        let r = pos - y_width - u64::from(x) * u64::from(I::PIXEL_SIZE);

        (x, y, r)
    }

    /// Read the pixel pointed to by `pos` into buf. Returns
    /// the number of bytes read from the pixel into buf. Returns
    /// `None` if `pos` is out-of-bounds.
    fn read_pixel_to_buf(&mut self, buf: &mut [u8]) -> Option<usize> {
        let (pixel_x, pixel_y, pixel_offset) = self.xyrem_from_pos(self.pos);

        let pixel = self.image.get_pixel(pixel_x, pixel_y)?;
        #[allow(clippy::cast_possible_truncation)]
        let unread_pixel = &pixel[pixel_offset as usize..];

        let i = copy_min_len(unread_pixel, buf);
        Some(i)
    }

    /// Write `buf` to the pixel pointed by `pos`. Returns the
    /// number of bytes written from `buf` into the pixel. Returns
    /// `None` if `pos` is out-of-bounds.
    fn write_buf_to_pixel(&mut self, buf: &[u8]) -> Option<usize> {
        let (pixel_x, pixel_y, pixel_offset) = self.xyrem_from_pos(self.pos);

        let pixel = self.image.get_pixel_mut(pixel_x, pixel_y)?;
        #[allow(clippy::cast_possible_truncation)]
        let unwritten_pixel = &mut pixel[pixel_offset as usize..];

        let i = copy_min_len(buf, unwritten_pixel);
        Some(i)
    }
}

impl<I> Read for ImageCursor<I>
where
    I: Image,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0;

        loop {
            let i = match self.read_pixel_to_buf(&mut buf[bytes_read..]) {
                Some(i) if i == 0 => break,
                Some(i) => i,
                None => break,
            };

            bytes_read += i;
            self.seek(io::SeekFrom::Current(i as i64))?;
        }

        Ok(bytes_read)
    }
}

impl<I> Write for ImageCursor<I>
where
    I: Image,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes_written = 0;

        loop {
            let i = match self.write_buf_to_pixel(&buf[bytes_written..]) {
                Some(i) if i == 0 => break,
                Some(i) => i,
                None => break,
            };

            bytes_written += i;
            self.seek(io::SeekFrom::Current(i as i64))?;
        }

        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<I> Seek for ImageCursor<I>
where
    I: Image,
{
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        use io::SeekFrom;
        match pos {
            SeekFrom::Start(offset) => {
                self.pos = offset;
                Ok(self.pos)
            }
            SeekFrom::Current(offset) => {
                self.pos = self.pos.saturating_add_signed(offset);
                Ok(self.pos)
            }
            SeekFrom::End(offset) => {
                self.pos = self.capacity().saturating_add_signed(offset);
                Ok(self.pos)
            }
        }
    }
}

fn copy_min_len<T>(src: &[T], dst: &mut [T]) -> usize
where
    T: Copy,
{
    let i = core::cmp::min(src.len(), dst.len());
    dst[..i].copy_from_slice(&src[..i]);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rw_rgb8() {
        let mut cursor = ImageCursor::new(image::RgbImage::new(2, 1));

        assert_eq!(cursor.write(&[0x01, 0x02]).unwrap(), 2);
        assert_eq!(cursor.write(&[0x03, 0x04, 0x05]).unwrap(), 3);
        assert_eq!(cursor.write(&[0x06, 0x07, 0x08, 0x09]).unwrap(), 1);

        cursor.seek(io::SeekFrom::Start(0)).unwrap();
        let mut buf = vec![0u8; 6];

        assert_eq!(cursor.read(&mut buf[0..2]).unwrap(), 2);
        assert_eq!(&buf[0..2], &[0x01, 0x02]);

        assert_eq!(cursor.read(&mut buf[2..]).unwrap(), 4);
        assert_eq!(&buf[2..], &[0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn test_rw_rgba32() {
        let data = (0x0..=0xf).collect::<Vec<_>>();

        let mut cursor = ImageCursor::new(image::Rgba32FImage::new(1, 1));

        assert_eq!(cursor.write(&data).unwrap(), 16);
        assert_eq!(cursor.write(&[0x00]).unwrap(), 0);

        cursor.seek(io::SeekFrom::Start(0)).unwrap();
        let mut buf = vec![0u8; 16];

        assert_eq!(cursor.read(&mut buf).unwrap(), 16);
        assert_eq!(buf, data);

        assert_eq!(cursor.read(&mut buf).unwrap(), 0);
        assert_eq!(buf, data);
    }
}
