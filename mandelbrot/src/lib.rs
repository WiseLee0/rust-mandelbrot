extern crate wasm_bindgen;
use image::GrayImage;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[derive(Clone, Copy)]
struct Complex<T> {
    re: T,
    im: T,
}

/// 尝试测定`c`是否位于曼德博集中，使用最多`limit`次迭代来判定
///
/// 如果`c`不是集合成员之一，则返回`Some(i)`，其中的`i`是`c`离开以原点
/// 为中心的半径为2的圆时所需的迭代次数。如果`c`似乎是集合成员之一（确
/// 切而言是达到了迭代次数限制但仍然无法证明`c`不是成员），则返回`None`
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.re * z.re + z.im * z.im > 4.0 {
            return Some(i);
        }
        let temp = z.re * z.re - z.im * z.im + c.re;
        z.im = 2.0 * z.re * z.im + c.im;
        z.re = temp;
    }
    None
}

/// 给定输出图像中像素的行和列，返回复平面中对应的坐标
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + ((pixel.0 as f64) * width) / (bounds.0 as f64),
        im: upper_left.im - ((pixel.1 as f64) * height) / (bounds.1 as f64),
    }
}

/// 将曼德博集对应的矩形渲染到像素缓冲区中
///
/// `bounds`参数会给出缓冲区`pixels`的宽度和高度，缓冲区的每字节都包含一个灰度像素。
#[wasm_bindgen]
pub fn render(width: usize, height: usize) -> Vec<u8> {
    let bounds = (width, height);
    let mut pixels = vec![0; bounds.0 * bounds.1];
    let upper_left = Complex { re: -1.2, im: 0.35 };
    let lower_right = Complex { re: -1.0, im: 0.2 };
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - (count as u8),
            };
        }
    }
    pixels
}

#[wasm_bindgen]
pub fn parallel_render(width: usize, height: usize) -> Vec<u8> {
    let bounds = (width, height);
    let mut pixels = vec![0; bounds.0 * bounds.1];

    pixels
        .par_chunks_mut(bounds.0)
        .enumerate()
        .for_each(|(row, band)| {
            let upper_left = Complex { re: -1.2, im: 0.35 };
            let lower_right = Complex { re: -1.0, im: 0.2 };
            let top = row;
            let band_bounds = (bounds.0, 1);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);

            for column in 0..band_bounds.0 {
                let point =
                    pixel_to_point(band_bounds, (column, 0), band_upper_left, band_lower_right);
                band[column] = match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - (count as u8),
                };
            }
        });

    pixels
}

#[wasm_bindgen]
/// 返回数据指针，内存共享的方式避免值的传递拷贝
pub fn render_shared(width: usize, height: usize) -> *const u8 {
    let bounds = (width, height);
    let pixel_length = bounds.0 * bounds.1;
    let mut pixels = Vec::with_capacity(pixel_length);
    let upper_left = Complex { re: -1.2, im: 0.35 };
    let lower_right = Complex { re: -1.0, im: 0.2 };

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels.push(match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            });
        }
    }

    let pixels_box = pixels.into_boxed_slice();
    let ptr = pixels_box.as_ptr();
    Box::into_raw(pixels_box);

    ptr
}

pub fn save_gray_image(pixels: Vec<u8>, width: usize, height: usize) -> () {
    let img = GrayImage::from_raw(width as u32, height as u32, pixels).unwrap();
    let _ = img.save("./test.png");
}

#[test]
fn test_render() {
    let bounds = (4000, 3000);
    let pixels = render(bounds.0, bounds.1);
    save_gray_image(pixels, bounds.0, bounds.1);
}

#[test]
fn test_parallel_render() {
    let bounds = (4000, 3000);
    let pixels = parallel_render(bounds.0, bounds.1);
    save_gray_image(pixels, bounds.0, bounds.1);
}
