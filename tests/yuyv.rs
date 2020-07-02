use std::convert::TryFrom;
use std::ops::RangeInclusive;

use ffimage::color::*;
use ffimage::core::{Convert, ImageView, TryConvert};
use ffimage::packed::{
    DynamicImageView, GenericImageBuffer, GenericImageFlatBuffer, GenericImageView,
};

use ffimage_yuv::yuv::*;
use ffimage_yuv::yuyv::*;

fn make_range(val: u8, delta: u8) -> RangeInclusive<u8> {
    let lower = if val <= delta { 0 } else { val - delta };
    let upper = if val >= 255 - delta { 255 } else { val + delta };

    lower..=upper
}

#[test]
fn convert_yuy_to_yuyv() {
    let mem: [u8; 12] = [10; 12];
    let view = GenericImageView::<Yuv<u8>>::new(&mem, 2, 2).unwrap();
    let mut buf = GenericImageBuffer::<Yuyv<u8>>::new(0, 0);
    view.convert(&mut buf);

    for i in 0..view.height() {
        for j in (0..view.width()).step_by(2) {
            let pix_in = [view.get_pixel(j, i).unwrap(), view.get_pixel(j + 1, i).unwrap()];
            let pix_out = buf.get_pixel(j, i).unwrap();

            // one macropixel is two image pixels
            assert_eq!(pix_out[0], pix_in[0][0]);
            assert_eq!(pix_out[1], pix_in[0][1]);
            assert_eq!(pix_out[2], pix_in[0][1]);
            assert_eq!(pix_out[3], pix_in[0][2]);
        }
    }
}

#[test]
fn convert_yuyv_to_yuv() {
    let mem: [u8; 12] = [10; 12];
    let view = GenericImageView::<Yuyv<u8>>::new(&mem, 2, 2).unwrap();
    let mut buf = GenericImageBuffer::<Yuv<u8>>::new(0, 0);
    view.convert(&mut buf);

    for i in 0..view.height() {
        for j in (0..view.width()).step_by(2) {
            let pix_in = view.get_pixel(j, i).unwrap();
            let pix_out = [buf.get_pixel(j, i).unwrap(), buf.get_pixel(j + 1, i).unwrap()];

            // one macropixel is two image pixels
            assert_eq!(pix_out[0][0], pix_in[0]);
            assert_eq!(pix_out[0][1], pix_in[1]);
            assert_eq!(pix_out[1][0], pix_in[2]);
            assert_eq!(pix_out[0][2], pix_in[3]);
        }
    }
}

#[test]
fn convert_yuyv_rgb() {
    let mut mem: [u8; 12] = [10; 12];
    mem[0] = 111;
    mem[1] = 222;
    mem[2] = 255;
    mem[3] = 11;
    mem[4] = 22;
    mem[5] = 33;
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 2, 2).unwrap();
    let mut yuv_buf = GenericImageBuffer::<Yuv<u8>>::new(0, 0);
    let mut yuyv_buf = GenericImageBuffer::<Yuyv<u8>>::new(0, 0);
    let mut rgb_buf = GenericImageBuffer::<Rgb<u8>>::new(0, 0);
    view.convert(&mut yuv_buf);
    yuv_buf.convert(&mut yuyv_buf);
    yuyv_buf.convert(&mut yuv_buf);
    yuv_buf.convert(&mut rgb_buf);
}
