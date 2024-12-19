use image::{GrayImage, Luma};
use std::f64::consts::PI;

const GRID_SIZE: u32 = 8; // 8x8 cosine functions
const BLOCK_SIZE: u32 = 8; // 8x8 pixel image block
const SCALE: u32 = 40;
const BORDER_SIZE: u32 = 20;

pub fn generate_basis_img() {
    // total image size including borders
    let total_size = BLOCK_SIZE * SCALE * GRID_SIZE + BORDER_SIZE * (GRID_SIZE + 1);
    let mut img = GrayImage::new(total_size, total_size);

    // draw white background for borders
    for y in 0..total_size {
        for x in 0..total_size {
            img.put_pixel(x, y, Luma([255]));
        }
    }

    for v in 0..GRID_SIZE {
        for u in 0..GRID_SIZE {
            // position of current block
            let block_x = u as u32 * (BLOCK_SIZE * SCALE + BORDER_SIZE) + BORDER_SIZE;
            let block_y = v as u32 * (BLOCK_SIZE * SCALE + BORDER_SIZE) + BORDER_SIZE;

            // make sure the first one is just pure white
            if u == 0 && v == 0 {
                for y in 0..BLOCK_SIZE {
                    for x in 0..BLOCK_SIZE {
                        for sy in 0..SCALE {
                            for sx in 0..SCALE {
                                img.put_pixel(
                                    block_x + x * SCALE + sx,
                                    block_y + y * SCALE + sy,
                                    Luma([255]),
                                );
                            }
                        }
                    }
                }
                continue;
            }

            // draw basis function
            for y in 0..BLOCK_SIZE {
                for x in 0..BLOCK_SIZE {
                    let basis =
                        (((2.0 * x as f64 + 1.0) * u as f64 * PI) / (2.0 * GRID_SIZE as f64)).cos()
                            * (((2.0 * y as f64 + 1.0) * v as f64 * PI) / (2.0 * GRID_SIZE as f64))
                                .cos();

                    // scale to 0-255
                    let value = ((basis + 1.0) * 127.5) as u8;

                    // draw pixels
                    for sy in 0..SCALE {
                        for sx in 0..SCALE {
                            img.put_pixel(
                                block_x + x * SCALE + sx,
                                block_y + y * SCALE + sy,
                                Luma([value]),
                            );
                        }
                    }
                }
            }
        }
    }

    img.save("dct_basis.png").unwrap();
}
