use image::imageops;
use image::imageops::colorops;
use image::io::Reader;
use image::{GenericImageView, ImageBuffer, Pixel};
extern crate clap;
use clap::{App, Arg};

fn constrained_resize_dims(target: (u32, u32), orig: (u32, u32)) -> (u32, u32) {
    let target_width = target.0 as f32;
    let target_height = target.1 as f32;
    let orig_width = orig.0 as f32;
    let orig_height = orig.1 as f32;

    if target_width / target_height < orig_width / orig_height {
        // constrain the height to target and adjust the width accordingly
        // if the target w:h ratio is < orig w:h ratio
        // aka the target is skinnier than the orig
        // then constrain the height to the target and let width adjust
        (((orig_width / orig_height) * target_height) as u32, target.1)
    } else {
        // else the target is fatter orig so constrain the width to target
        (target.0, ((orig_height / orig_width) * target_width) as u32)
    }
}

fn resize_crop<I: GenericImageView>(
    img: &I,
    w: u32,
    h: u32,
) -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where
    I::Pixel: 'static,
    <I::Pixel as Pixel>::Subpixel: 'static,
{
    let (width, height) = img.dimensions();
    let (resize_w, resize_h) = constrained_resize_dims((w, h), (width, height));
    let resized = imageops::resize(img, resize_w, resize_h, imageops::FilterType::Gaussian);
    // crop in the middle
    let x = (resize_w / 2) - (w / 2);
    let cropped = imageops::crop_imm(&resized, x, 0, w, h);
    cropped.to_image()
}

fn main() {
    let matches = App::new("E-Ink Gallery Maker")
        .version("6000000000")
        .author("bballant")
        .about("Make jpgs for display on raspberry pi e-ink display.")
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .index(1)
                .help("Input file"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .required(true)
                .index(2)
                .help("Output file"),
        )
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();

    let img = Reader::open(input_file)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let cropped = resize_crop(&img, 122, 255);
    let grey = colorops::grayscale(&cropped);
    let out = colorops::contrast(&grey, 100.0);
    out.save(output_file).unwrap();
    println!("Done!");
}
