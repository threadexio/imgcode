use std::mem;

use crate::private::Sealed;

pub trait Image: Sealed {
    type ChannelType;
    const CHANNEL_NUM: u32;

    #[allow(clippy::cast_possible_truncation)]
    const PIXEL_SIZE: u32 = (mem::size_of::<Self::ChannelType>() as u32) * Self::CHANNEL_NUM;

    fn new_with_dimensions(x: u32, y: u32) -> Self
    where
        Self: Sized;

    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]>;
    fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut [u8]>;
}

mod impls {
    use std::mem;
    use std::slice;

    use crate::private::Sealed;
    use crate::traits::Image;

    impl Sealed for image::RgbImage {}
    impl Image for image::RgbImage {
        type ChannelType = u8;
        const CHANNEL_NUM: u32 = 3;

        fn new_with_dimensions(x: u32, y: u32) -> Self
        where
            Self: Sized,
        {
            Self::new(x, y)
        }

        fn width(&self) -> u32 {
            self.width()
        }

        fn height(&self) -> u32 {
            self.height()
        }

        fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
            self.get_pixel_checked(x, y).map(|x| x.0.as_slice())
        }

        fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut [u8]> {
            self.get_pixel_mut_checked(x, y).map(|x| x.0.as_mut_slice())
        }
    }

    impl Sealed for image::RgbaImage {}
    impl Image for image::RgbaImage {
        type ChannelType = u8;
        const CHANNEL_NUM: u32 = 4;

        fn new_with_dimensions(x: u32, y: u32) -> Self
        where
            Self: Sized,
        {
            Self::new(x, y)
        }

        fn width(&self) -> u32 {
            self.width()
        }

        fn height(&self) -> u32 {
            self.height()
        }

        fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
            self.get_pixel_checked(x, y).map(|x| x.0.as_slice())
        }

        fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut [u8]> {
            self.get_pixel_mut_checked(x, y).map(|x| x.0.as_mut_slice())
        }
    }

    impl Sealed for image::Rgb32FImage {}
    impl Image for image::Rgb32FImage {
        type ChannelType = f32;
        const CHANNEL_NUM: u32 = 3;

        fn new_with_dimensions(x: u32, y: u32) -> Self
        where
            Self: Sized,
        {
            Self::new(x, y)
        }

        fn width(&self) -> u32 {
            self.width()
        }

        fn height(&self) -> u32 {
            self.height()
        }

        fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
            self.get_pixel_checked(x, y)
                .map(|x| slice_to_u8_slice(&x.0))
        }

        fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut [u8]> {
            self.get_pixel_mut_checked(x, y)
                .map(|x| slice_to_u8_slice_mut(&mut x.0))
        }
    }

    impl Sealed for image::Rgba32FImage {}
    impl Image for image::Rgba32FImage {
        type ChannelType = f32;
        const CHANNEL_NUM: u32 = 4;

        fn new_with_dimensions(x: u32, y: u32) -> Self
        where
            Self: Sized,
        {
            Self::new(x, y)
        }

        fn width(&self) -> u32 {
            self.width()
        }

        fn height(&self) -> u32 {
            self.height()
        }

        fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
            self.get_pixel_checked(x, y)
                .map(|x| slice_to_u8_slice(&x.0))
        }

        fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut [u8]> {
            self.get_pixel_mut_checked(x, y)
                .map(|x| slice_to_u8_slice_mut(&mut x.0))
        }
    }

    /// Convert an `f32` slice into a `u8` slice that takes up the same memory.
    fn slice_to_u8_slice<'a, T>(slice: &'a [T]) -> &'a [u8]
    where
        T: Sized,
    {
        let len = mem::size_of_val(slice);
        let ptr = slice.as_ptr().cast::<u8>();

        unsafe { slice::from_raw_parts::<'a, u8>(ptr, len) }
    }

    /// Mutable version of [`slice_to_u8_slice`].
    fn slice_to_u8_slice_mut<'a, T>(slice: &'a mut [T]) -> &'a mut [u8]
    where
        T: Sized,
    {
        let len = mem::size_of_val(slice);
        let ptr = slice.as_mut_ptr().cast::<u8>();

        unsafe { slice::from_raw_parts_mut::<'a, u8>(ptr, len) }
    }
}
