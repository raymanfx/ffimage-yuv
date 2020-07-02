use std::array;
use std::convert::TryFrom;

use num_traits::{AsPrimitive, FromPrimitive};

use ffimage::color::rgb::*;
use ffimage::core::traits::{Pixel, StorageType, TryConvertSlice};
use ffimage::core::PixelRow;
use ffimage::{create_macropixel, define_pixel, impl_Pixel};

use crate::yuv::*;

create_macropixel!(Yuyv, 4, 2, #[doc = "Yuyv macropixel"]);

impl<T: StorageType> From<Yuyv<T>> for [Yuv<T>; 2] {
    fn from(pix: Yuyv<T>) -> Self {
        let _1 = Yuv {
            0: [pix[0], pix[1], pix[3]],
        };
        let _2 = Yuv {
            0: [pix[2], pix[1], pix[3]],
        };

        [_1, _2]
    }
}

impl<T: StorageType> From<[Yuv<T>; 2]> for Yuyv<T> {
    fn from(pix: [Yuv<T>; 2]) -> Self {
        Yuyv {
            0: [pix[0][0], pix[0][1], pix[1][0], pix[0][2]],
        }
    }
}

impl<T: StorageType> TryConvertSlice<Yuv<T>> for [Yuyv<T>] {
    type Error = ();

    fn try_convert(&self, output: &mut [Yuv<T>]) -> Result<(), Self::Error> {
        // one YUYV macropixel gives two YUV image pixels
        if self.len() * 2 != output.len() {
            return Err(());
        }

        for i in 0..self.len() {
            let pixels = <[Yuv<T>; 2]>::from(self[i]);
            let j = i * 2;
            output[j] = pixels[0];
            output[j + 1] = pixels[1];
        }

        Ok(())
    }
}

impl<T: StorageType> TryConvertSlice<Yuyv<T>> for [Yuv<T>] {
    type Error = ();

    fn try_convert(&self, output: &mut [Yuyv<T>]) -> Result<(), Self::Error> {
        // one YUYV macropixel gives two YUV image pixels
        if self.len() != output.len() * 2 {
            return Err(());
        }

        for i in (0..self.len()).step_by(2) {
            let pixels = [self[i], self[i + 1]];
            let j = i / 2;
            output[j] = Yuyv::<T>::from(pixels);
        }

        Ok(())
    }
}

/*
impl<T: StorageType> TryConvertSlice<Rgb<T>> for [Yuyv<T>] {
    type Error = ();

    fn try_convert(&self, output: &mut [Rgb<T>]) -> Result<(), Self::Error> {
        let yuv = <[Yuv<T>]>::from(self)?;
        yuv.try_convert(&mut output)
    }
}

impl<T: StorageType> TryConvertSlice<Yuyv<T>> for [Rgb<T>] {
    type Error = ();

    fn try_convert(&self, output: &mut [Yuyv<T>]) -> Result<(), Self::Error> {
        let yuv = <[Yuv<T>]>::from(self)?;
        yuv.try_convert(&mut output)
    }
}
*/

impl<
        I: StorageType + AsPrimitive<i32> + AsPrimitive<O> + FromPrimitive,
        O: StorageType + FromPrimitive + 'static,
    > From<Yuyv<I>> for [Rgb<O>; 2]
{
    fn from(pix: Yuyv<I>) -> Self {
        let pix = <[Yuv<I>; 2]>::from(pix);

        [Rgb::<O>::from(pix[0]), Rgb::<O>::from(pix[1])]
    }
}

impl<
        I: StorageType + AsPrimitive<i32> + AsPrimitive<O> + FromPrimitive,
        O: StorageType + FromPrimitive + 'static,
    > From<[Rgb<I>; 2]> for Yuyv<O>
{
    fn from(pix: [Rgb<I>; 2]) -> Self {
        let _1 = Yuv::<I>::from(pix[0]);
        let _2 = Yuv::<I>::from(pix[1]);

        Yuyv {
            0: [_1.0[0].as_(), _1.0[1].as_(), _2.0[0].as_(), _1.0[2].as_()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let pix: Yuyv<u8> = Yuyv { 0: [255; 4] };

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(1), 255);
        assert_eq!(pix.at(2), 255);
        assert_eq!(pix.at(3), 255);
    }

    #[test]
    fn cast_from_slice() {
        let mem = vec![255; 4];
        let pix = Yuyv::<u8>::cast_from_slice(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(1), 255);
        assert_eq!(pix.at(2), 255);
        assert_eq!(pix.at(3), 255);
    }

    #[test]
    fn cast_from_slice_mut() {
        let mut mem = vec![255; 4];
        let pix = Yuyv::<u8>::cast_from_slice_mut(&mut mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(1), 255);
        assert_eq!(pix.at(2), 255);
        assert_eq!(pix.at(3), 255);
    }

    #[test]
    fn try_from() {
        let mem = vec![255; 4];
        let pix: Yuyv<u8> = Pixel::try_from(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(1), 255);
        assert_eq!(pix.at(2), 255);
        assert_eq!(pix.at(3), 255);
    }

    #[test]
    fn channels() {
        assert_eq!(Yuyv::<u8>::channels(), 4);
    }
}
