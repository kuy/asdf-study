use asdf_pixel_sort::sort;

const BLACK: u32 = 48;

fn main() {
    let img = image::open("ASDFPixelSort/pic.jpg").unwrap();
    let mut buf = img.to_rgb8();

    sort(&mut buf, BLACK);

    buf.save("sorted.png").unwrap();
}
