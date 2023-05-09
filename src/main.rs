use image::imageops;
use image::{ImageBuffer, Pixel};
use std::{fs::File, io::BufWriter};
use visual_center::img_processor::ImgProcessor;

fn visual_center(img: image::DynamicImage) -> image::DynamicImage {
    let img_processor = ImgProcessor::new(img);
    let pixel_count = img_processor.count_pixels();
    let (x_sum, _) = img_processor.sum_coors();
    let (x_center, _) = img_processor.get_image_center();
    let x_avg = x_sum as f64 / pixel_count as f64;
    let shift = x_center as f64 - x_avg;
    let shifted_img = img_processor.move_pixels(shift.round() as i32);

    shifted_img
}

fn main() {
    let input_dir = "./input";
    let entries = std::fs::read_dir(input_dir).unwrap();
    let text_img = image::open("text.png").unwrap();

    let names = [
        "beagle",
        "bernese_mountain_dog",
        "boston_terrier",
        "brittany_spaniel",
        "corgi",
        "cream_french_bulldog",
        "dachshund",
        "english_bulldog",
        "goldendoodle",
        "havanese",
        "husky",
        "jack_russell_terrier",
        "miniature_pinscher",
        "rottweiler",
        "samoyed",
        "shiba_inu",
        "schnauzer",
        "westie",
    ];

    for entry in entries {
        let path = entry.unwrap().path();

        if let Some("png") = path.extension().and_then(|s| s.to_str()) {
            let input_img = image::open(&path).unwrap();
            let mut shifted_img = visual_center(input_img);

            imageops::overlay(&mut shifted_img, &text_img, 0, 0);

            let extracted_name = extract_name(&names, path.file_name().unwrap().to_str().unwrap());
            let output_path = format!("./output/{}.png", extracted_name.unwrap());
            let output_file = std::fs::File::create(&output_path).unwrap();

            save_png_with_dpi(output_file, &shifted_img.into_rgba8(), 300).unwrap();
        }
    }
}

/// Writes the PNG image data to the given `file` and assigns the given
/// `dpi` to the image.
///
/// Arguments:
///
/// * `file`: The file to which the image data is written.
/// * `imgbuf`: The [ImageBuffer] containing the unencoded image data.
/// * `dpi`: The **dots per inch** which will be assigned to the image.
///
/// Returns:
///
/// A [Result<(), std::io::Error>] indicating if the operation was successful.
fn save_png_with_dpi<P>(
    file: File,
    imgbuf: &ImageBuffer<P, Vec<u8>>,
    dpi: u32,
) -> Result<(), std::io::Error>
where
    P: Pixel<Subpixel = u8>,
{
    let w = &mut BufWriter::new(file);
    let (width, height) = imgbuf.dimensions();
    let mut encoder = png::Encoder::new(w, width, height);

    match <P as Pixel>::CHANNEL_COUNT {
        4 => encoder.set_color(png::ColorType::Rgba),
        3 => encoder.set_color(png::ColorType::Rgb),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Incorrect color channel count / format.",
            ))
        }
    }

    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Best);

    let mut writer = encoder.write_header()?;
    let data = get_dpi_header_data(dpi);

    writer.write_chunk(png::chunk::pHYs, data.as_slice())?;
    writer.write_image_data(imgbuf)?;
    Ok(())
}

/// Converts the given `dpi` value to the appropriate _pHYs_ chunk data.
///
/// Arguments:
///
/// * `dpi`: The **dots per inch** of the image.
///
/// Returns:
///
/// A [Vec<u8>] with the encoded chunk data.
fn get_dpi_header_data(dpi: u32) -> Vec<u8> {
    let dpm = 39.370079 * dpi as f32; // Convert from dots per inch to dots per meter.
    let rounded_dpm = dpm.round() as u32;

    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(&rounded_dpm.to_be_bytes()); // Pixels per unit in X-direction.
    data.extend_from_slice(&rounded_dpm.to_be_bytes()); // Pixels per unit in Y-direction.

    data.push(1); // Indicate that meters are used as unit.
    data
}

fn extract_name<'a>(array: &'a [&'a str], string: &'a str) -> Option<&'a str> {
    for name in array {
        if string.contains(name) {
            return Some(name);
        }
    }
    None
}
