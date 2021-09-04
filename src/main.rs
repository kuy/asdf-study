mod finder;

const BLACK: u32 = 32;

fn main() {
    let img = image::open("ASDFPixelSort/pic.jpg").unwrap();
    let mut buf = img.to_rgb8();

    for col in 0..buf.width() {
        sort_column(&mut buf, col);
    }

    for row in 0..buf.height() {
        sort_row(&mut buf, row);
    }

    buf.save("sorted.png").unwrap();
}

fn sort_column(buf: &mut image::RgbImage, x: u32) {
    let height = buf.height();

    let mut y = 0;
    let mut y_end = 0;

    while y_end < height - 1 {
        y = match finder::get_first_not_black_y(buf, x, y, BLACK) {
            Some(y) => y,
            _ => break,
        };

        y_end = finder::get_next_black_y(buf, x, y, BLACK);

        let len = y_end - y;
        let mut line = Vec::with_capacity(len as usize);

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

fn sort_row(buf: &mut image::RgbImage, y: u32) {
    let width = buf.width();

    let mut x = 0;
    let mut x_end = 0;

    while x_end < width - 1 {
        x = match finder::get_first_not_black_x(buf, x, y, BLACK) {
            Some(x) => x,
            _ => break,
        };

        x_end = finder::get_next_black_x(buf, x, y, BLACK);

        let len = x_end - x;
        let mut line = Vec::with_capacity(len as usize);

        for x in x..x_end {
            let pixel = buf.get_pixel(x, y);
            line.push(*pixel);
        }

        line.sort_by(|a, b| to_gray(a).cmp(&to_gray(b)));

        for i in 0..len {
            let pixel = line.get(i as usize).unwrap();
            buf.put_pixel(x + i, y, *pixel);
        }

        x = x_end + 1;
    }
}

pub(crate) fn to_gray(color: &image::Rgb<u8>) -> u32 {
    (color.0[0] as u32 + color.0[1] as u32 + color.0[2] as u32) / 3
}
