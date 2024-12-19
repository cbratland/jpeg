use image::{DynamicImage, ImageBuffer, Rgb};
use std::f64::consts::PI;

const N: usize = 8;

const LUMINANCE_TABLE: [[u8; N]; N] = [
    [16, 11, 10, 16, 24, 40, 51, 61],
    [12, 12, 14, 19, 26, 58, 60, 55],
    [14, 13, 16, 24, 40, 57, 69, 56],
    [14, 17, 22, 29, 51, 87, 80, 62],
    [18, 22, 37, 56, 68, 109, 103, 77],
    [24, 35, 55, 64, 81, 104, 113, 92],
    [49, 64, 78, 87, 103, 121, 120, 101],
    [72, 92, 95, 98, 112, 100, 103, 99],
];

const CHROMINANCE_TABLE: [[u8; N]; N] = [
    [17, 18, 24, 47, 99, 99, 99, 99],
    [18, 21, 26, 66, 99, 99, 99, 99],
    [24, 26, 56, 99, 99, 99, 99, 99],
    [47, 66, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
];

fn dct_2d(block: &[[f64; N]; N]) -> [[f64; N]; N] {
    let mut result = [[0.0; N]; N];

    let scale_factor = 2.0 / N as f64;

    for u in 0..N {
        for v in 0..N {
            let mut sum = 0.0;

            let au = if u == 0 { 1.0 / f64::sqrt(2.0) } else { 1.0 };
            let av = if v == 0 { 1.0 / f64::sqrt(2.0) } else { 1.0 };

            for n in 0..N {
                for m in 0..N {
                    sum += block[n][m]
                        * (PI * u as f64 * (2.0 * n as f64 + 1.0) / (2.0 * N as f64)).cos()
                        * (PI * v as f64 * (2.0 * m as f64 + 1.0) / (2.0 * N as f64)).cos();
                }
            }

            result[u][v] = scale_factor * au * av * sum;
        }
    }

    result
}

fn idct_2d(block: &[[f64; N]; N]) -> [[f64; N]; N] {
    let mut result = [[0.0; N]; N];

    let scale_factor = 2.0 / N as f64;

    for n in 0..N {
        for m in 0..N {
            let mut sum = 0.0;

            for u in 0..N {
                for v in 0..N {
                    let au = if u == 0 { 1.0 / f64::sqrt(2.0) } else { 1.0 };
                    let av = if v == 0 { 1.0 / f64::sqrt(2.0) } else { 1.0 };

                    sum += au
                        * av
                        * block[u][v]
                        * (PI * u as f64 * (2.0 * n as f64 + 1.0) / (2.0 * N as f64)).cos()
                        * (PI * v as f64 * (2.0 * m as f64 + 1.0) / (2.0 * N as f64)).cos();
                }
            }

            result[n][m] = sum * scale_factor;
        }
    }

    result
}

fn quantize(block: &[[f64; N]; N], quality: f64, is_luma: bool) -> [[f64; N]; N] {
    let mut result = [[0.0; N]; N];

    let scale = if quality <= 50.0 {
        50.0 / quality
    } else {
        2.0 - (quality / 50.0)
    };

    let qtable = if is_luma {
        &LUMINANCE_TABLE
    } else {
        &CHROMINANCE_TABLE
    };

    for i in 0..N {
        for j in 0..N {
            let qval = (qtable[i][j] as f64 * scale).max(1.0);
            result[i][j] = (block[i][j] / qval).round();
        }
    }

    result
}

fn dequantize(block: &[[f64; N]; N], quality: f64, is_luma: bool) -> [[f64; N]; N] {
    let mut result = [[0.0; N]; N];

    let scale = if quality <= 50.0 {
        50.0 / quality
    } else {
        2.0 - (quality / 50.0)
    };

    let qtable = if is_luma {
        &LUMINANCE_TABLE
    } else {
        &CHROMINANCE_TABLE
    };

    for i in 0..N {
        for j in 0..N {
            let qval = (qtable[i][j] as f64 * scale).max(1.0);
            result[i][j] = (block[i][j] * qval).round();
        }
    }

    result
}

fn rgb_to_ycbcr(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let cb = -0.169 * r - 0.331 * g + 0.500 * b;
    let cr = 0.500 * r - 0.419 * g - 0.081 * b;
    (y, cb, cr)
}

fn ycbcr_to_rgb(y: f64, cb: f64, cr: f64) -> (f64, f64, f64) {
    let r = y + 1.402 * cr;
    let g = y - 0.344136 * cb - 0.714136 * cr;
    let b = y + 1.772 * cb;
    (
        r.clamp(0.0, 255.0),
        g.clamp(0.0, 255.0),
        b.clamp(0.0, 255.0),
    )
}

pub fn compress_image(img: &DynamicImage, quality: f64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let mut output_buffer = image::ImageBuffer::new(width, height);

    for y in (0..height).step_by(N) {
        for x in (0..width).step_by(N) {
            let mut y_block = [[0.0; N]; N];
            let mut cb_block = [[0.0; N]; N];
            let mut cr_block = [[0.0; N]; N];

            // convert to ycbcr
            for by in 0..N {
                for bx in 0..N {
                    let px = x + bx as u32;
                    let py = y + by as u32;
                    if px < width && py < height {
                        let pixel = rgb_img.get_pixel(px, py);
                        let (y_val, cb_val, cr_val) =
                            rgb_to_ycbcr(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64);
                        y_block[by][bx] = y_val - 128.0;
                        cb_block[by][bx] = cb_val;
                        cr_block[by][bx] = cr_val;
                    }
                }
            }

            // perform dct and quantization, and reverse, for each component
            let y_compressed = {
                let dct = dct_2d(&y_block);
                let quantized = quantize(&dct, quality, true);
                let dequantized = dequantize(&quantized, quality, true);
                idct_2d(&dequantized)
            };

            let cb_compressed = {
                let dct = dct_2d(&cb_block);
                let quantized = quantize(&dct, quality, true);
                let dequantized = dequantize(&quantized, quality, true);
                idct_2d(&dequantized)
            };

            let cr_compressed = {
                let dct = dct_2d(&cr_block);
                let quantized = quantize(&dct, quality, true);
                let dequantized = dequantize(&quantized, quality, true);
                idct_2d(&dequantized)
            };

            // convert back to rgb and write to output
            for by in 0..N {
                for bx in 0..N {
                    let px = x + bx as u32;
                    let py = y + by as u32;
                    if px < width && py < height {
                        let (r, g, b) = ycbcr_to_rgb(
                            y_compressed[by][bx] + 128.0,
                            cb_compressed[by][bx],
                            cr_compressed[by][bx],
                        );

                        *output_buffer.get_pixel_mut(px, py) =
                            image::Rgb([r as u8, g as u8, b as u8]);
                    }
                }
            }
        }
    }

    output_buffer
}
