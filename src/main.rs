use asdf_pixel_sort::{sort_with_options, Mode, Options};

fn main() {
    let img = image::open("ASDFPixelSort/pic.jpg").unwrap();
    let mut buf = img.to_rgb8();

    let options = Options {
        mode: Mode::black(),
    };
    sort_with_options(&mut buf, &options);

    buf.save("sorted.png").unwrap();
}
