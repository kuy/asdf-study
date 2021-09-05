use asdf_pixel_sort::{sort, PColor};
use once_cell::sync::Lazy;

static BLACK: Lazy<PColor> = Lazy::new(|| PColor::new(11, 220, 0));

fn main() {
    let img = image::open("ASDFPixelSort/pic.jpg").unwrap();
    let mut buf = img.to_rgb8();

    sort(&mut buf, &BLACK);

    buf.save("sorted.png").unwrap();
}
