use criterion::{criterion_group, Criterion};

use ffimage::color::*;
use ffimage::core::Convert;
use ffimage::packed::{GenericImageBuffer, GenericImageView};

use ffimage_yuv::yuv::*;
use ffimage_yuv::yuyv::*;

pub fn rgb_to_yuv(c: &mut Criterion) {
    let mem: Vec<u8> = vec![0; 640 * 480 * 3];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 640, 480).unwrap();
    let mut buf = GenericImageBuffer::<Yuv<u8>>::new(640, 480);
    c.bench_function("Rgb[u8] -> Yuv[u8] (640x480)", |b| {
        b.iter(|| view.convert(&mut buf))
    });

    let mem: Vec<u8> = vec![0; 1280 * 720 * 3];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 1280, 720).unwrap();
    let mut buf = GenericImageBuffer::<Yuv<u8>>::new(1280, 720);
    c.bench_function("Rgb[u8] -> Yuv[u8] (1280x720)", |b| {
        b.iter(|| view.convert(&mut buf))
    });
}

pub fn rgb_to_yuyv(c: &mut Criterion) {
    let mem: Vec<u8> = vec![0; 640 * 480 * 3];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 640, 480).unwrap();
    let mut inter = GenericImageBuffer::<Yuv<u8>>::new(640, 480);
    let mut buf = GenericImageBuffer::<Yuyv<u8>>::new(640, 480);
    c.bench_function("Rgb[u8] -> Yuyv[u8] (640x480)", |b| {
        b.iter(|| {
            view.convert(&mut inter);
            inter.convert(&mut buf);
        })
    });

    let mem: Vec<u8> = vec![0; 1280 * 720 * 3];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 1280, 720).unwrap();
    let mut inter = GenericImageBuffer::<Yuv<u8>>::new(640, 480);
    let mut buf = GenericImageBuffer::<Yuyv<u8>>::new(1280, 720);
    c.bench_function("Rgb[u8] -> Yuyv[u8] (1280x720)", |b| {
        b.iter(|| {
            view.convert(&mut inter);
            inter.convert(&mut buf);
        })
    });
}

criterion_group!(benches, rgb_to_yuv, rgb_to_yuyv);
