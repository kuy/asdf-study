use crate::to_gray;

const BLACK: u32 = 4;

pub(crate) fn get_first_not_black_x(buf: &image::RgbImage, x_start: u32, y: u32) -> u32 {
    let mut x = x_start;

    while x < buf.width() {
        let pixel = buf.get_pixel(x, y);
        if BLACK <= to_gray(pixel) {
            break; // found non-black pixel
        }

        x += 1;
    }

    x
}

pub(crate) fn get_first_not_black_y(buf: &image::RgbImage, x: u32, y_start: u32) -> Option<u32> {
    let mut y = y_start;

    if y < buf.height() {
        loop {
            let pixel = buf.get_pixel(x, y);
            if BLACK <= to_gray(pixel) {
                break; // found non-black pixel
            }

            y += 1;

            if buf.height() <= y {
                return None;
            }
        }
    }

    Some(y)
}

pub(crate) fn get_next_black_y(buf: &image::RgbImage, x: u32, y_start: u32) -> u32 {
    let height = buf.height();
    let mut y = y_start + 1;

    if y < height {
        loop {
            let pixel = buf.get_pixel(x, y);
            if to_gray(pixel) <= BLACK {
                break; // found black pixel
            }

            y += 1;

            if height <= y {
                return height - 1;
            }
        }
    }

    y - 1
}
