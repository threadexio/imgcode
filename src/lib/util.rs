use crate::traits::Image;

pub fn copy_min_len<T>(src: &[T], dst: &mut [T]) -> usize
where
    T: Copy,
{
    let i = core::cmp::min(src.len(), dst.len());
    dst[..i].copy_from_slice(&src[..i]);
    i
}

pub fn xyrem_from_pos<I>(pos: u64, width: u64) -> (u64, u64, u64)
where
    I: Image,
{
    let width = width * I::PIXEL_SIZE;

    let y = pos / width;
    let x = (pos - (y * width)) / I::PIXEL_SIZE;
    let r = pos - (y * width) - (x * I::PIXEL_SIZE);

    (x, y, r)
}
