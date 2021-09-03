use image::GenericImageView;

fn main() {
    let img = image::open("ASDFPixelSort/pic.jpg").unwrap();
    let mut buf = img.to_rgb8();

    for col in 0..buf.width() {
        // println!("col={}", col);
        sort_column(&mut buf, col);
    }

    buf.save("sorted.png").unwrap();
}

fn sort_column(buf: &mut image::RgbImage, x: u32) {
    let mut y = 0;
    let mut y_end = 0;

    while y_end < buf.height() - 1 {
        y = match get_first_not_black_y(buf, x, y) {
            Some(y) => y,
            _ => break,
        };

        y_end = get_next_black_y(buf, x, y);

        let len = y_end - y;
        let mut line = Vec::with_capacity(len as usize);

        // println!("  y={}, y_end={}, len={}", y, y_end, len);

        for y in y..y_end {
            let pixel = buf.get_pixel(x, y);
            line.push(*pixel);
        }

        line.sort_by(|a, b| to_gray(a).cmp(&to_gray(b)));

        for i in 0..len {
            let pixel = line.get(i as usize).unwrap();
            buf.put_pixel(x, y + i, *pixel);
        }

        y = y_end + 1;
    }
}

const BLACK: u32 = 4;

fn get_first_not_black_x(buf: &image::RgbImage, x_start: u32, y: u32) -> u32 {
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

fn get_first_not_black_y(buf: &image::RgbImage, x: u32, y_start: u32) -> Option<u32> {
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

fn get_next_black_y(buf: &image::RgbImage, x: u32, y_start: u32) -> u32 {
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

fn to_gray(color: &image::Rgb<u8>) -> u32 {
    (color.0[0] as u32 + color.0[1] as u32 + color.0[2] as u32) / 3
}
