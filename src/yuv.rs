use std::array;
use std::convert::TryFrom;

use num_traits::{AsPrimitive, FromPrimitive};

use ffimage::color::bgr::*;
use ffimage::color::rgb::*;
use ffimage::core::traits::{Pixel, StorageType};
use ffimage::{create_pixel, define_pixel, impl_Pixel};

macro_rules! impl_from_rgb_to_yuv {
    ($src:ident, $dst:ident, $r:expr, $g:expr, $b:expr) => {
        impl<I: StorageType + AsPrimitive<i32>, O: StorageType + FromPrimitive> From<$src<I>>
            for $dst<O>
        {
            fn from(pix: $src<I>) -> Self {
                let r = pix[$r].as_();
                let g = pix[$g].as_();
                let b = pix[$b].as_();

                let y = ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
                let u = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
                let v = ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;

                let y = O::from_i32(y).unwrap();
                let u = O::from_i32(u).unwrap();
                let v = O::from_i32(v).unwrap();
                $dst { 0: [y, u, v] }
            }
        }
    };
}

macro_rules! impl_from_yuv_to_rgb {
    ($src:ident, $dst:ident, $r:expr, $g:expr, $b:expr) => {
        impl<I: StorageType + AsPrimitive<i32>, O: StorageType + FromPrimitive> From<$src<I>>
            for $dst<O>
        {
            fn from(pix: $src<I>) -> Self {
                let y = pix[0].as_();
                let u = pix[1].as_();
                let v = pix[2].as_();
                let c = y - 16;
                let d = u - 128;
                let e = v - 128;

                let r = num_traits::clamp((298 * c + 409 * e + 128) >> 8, 0, 255);
                let g = num_traits::clamp((298 * c - 100 * d - 208 * e + 128) >> 8, 0, 255);
                let b = num_traits::clamp((298 * c + 516 * d + 128) >> 8, 0, 255);

                let r = O::from_i32(r).unwrap();
                let g = O::from_i32(g).unwrap();
                let b = O::from_i32(b).unwrap();

                let mut result = $dst { 0: [r, g, b] };
                result[$r] = r;
                result[$g] = g;
                result[$b] = b;
                result
            }
        }
    };
}

create_pixel!(Yuv, 3, #[doc = "YUV pixel"]);

impl_from_rgb_to_yuv!(Bgr, Yuv, 2, 1, 0);
impl_from_rgb_to_yuv!(Rgb, Yuv, 0, 1, 2);

impl_from_yuv_to_rgb!(Yuv, Bgr, 2, 1, 0);
impl_from_yuv_to_rgb!(Yuv, Rgb, 0, 1, 2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let pix: Yuv<u8> = Yuv { 0: [255; 3] };

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(1), 255);
        assert_eq!(pix.at(2), 255);
    }

    #[test]
    fn cast_from_slice() {
        let mem = vec![255; 3];
        let pix = Yuv::<u8>::cast_from_slice(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(1), 255);
        assert_eq!(pix.at(2), 255);
    }

    #[test]
    fn cast_from_slice_mut() {
        let mut mem = vec![255; 3];
        let pix = Yuv::<u8>::cast_from_slice_mut(&mut mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(1), 255);
        assert_eq!(pix.at(2), 255);
    }

    #[test]
    fn try_from() {
        let mem = vec![255; 3];
        let pix: Yuv<u8> = Pixel::try_from(&mem).unwrap();

        assert_eq!(pix.at(0), 255);
        assert_eq!(pix.at(1), 255);
        assert_eq!(pix.at(2), 255);
    }

    #[test]
    fn channels() {
        assert_eq!(Yuv::<u8>::channels(), 3);
    }
}
