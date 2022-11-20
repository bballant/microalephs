use clap::{App, Arg};
use glob::*;
use image::{
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    Pixel,
    imageops::{self, colorops},
    io::Reader
};
use std::path::PathBuf;

/// Given target dimensions and input dimensions, return new
/// dimensions for resizing the image so that the proportionally
/// longest dimension is constrained to the target dimension and the
/// other is rescaled proportionally.
/// This is the resize step in resize_crop().
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
        // else the target is fatter so constrain the width to target
        (target.0, ((orig_height / orig_width) * target_width) as u32)
    }
}

/// Resize and crop the given image to best fit the given dimensions
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

fn input_files(input_glob: &str) -> impl Iterator<Item = PathBuf> {
    glob(input_glob).unwrap().flatten()
}

fn main() {
    let matches = App::new("E-Ink Gallery Maker")
        .version("zero")
        .author("bballant")
        .about("Make pngs for display on raspberry pi e-ink display.")
        .arg(
            Arg::with_name("GLOB")
                .required(true)
                .index(1)
                .help("Input glob"),
        )
        .arg(
            Arg::with_name("OUTDIR")
                .required(true)
                .index(2)
                .help("Output directory"),
        )
        .get_matches();

    let input_glob = matches.value_of("GLOB").unwrap();
    let output_dir = matches.value_of("OUTDIR").unwrap();

    for (i, file) in input_files(input_glob).enumerate() {

        let filename: String =
            file
            .as_path()
            .file_stem()
            .and_then(|x| x.to_str())
            .map(|y| String::from(y))
            .unwrap_or(String::from("foo"));

        let img_opt: Option<DynamicImage> =
            Reader::open(&file)
            .and_then(|x| x.with_guessed_format())
            .ok()
            .and_then(|x| x.decode().ok());

        match img_opt {
            Some(img) => {
                let output_path = format!("{}/{:03}.{}.png", output_dir, i, filename);
                println!("Creating {}.", output_path);
                let img = resize_crop(&img, 122, 255);
                //let img = resize_crop(&img, 128, 64);
                let img = colorops::grayscale(&img);
                let img = colorops::brighten(&img, 20);
                let mut out = colorops::contrast(&img, 100.0);
                colorops::dither(&mut out, &colorops::BiLevel);
                println!("Saving {}.", output_path);
                match out.save(&output_path) {
                    Ok(_) => {
                        println!("Saved {}!", output_path);
                    },
                    Err(e) => {
                        println!("Could not save {}.", output_path);
                        println!("{:?}", e);
                    }
                }
            },
            None => {
                println!("Could not open {}.", file.to_str().unwrap_or("unknown"));
                continue;
            },
        }

    }
    println!("Done!");
}
