use image::{ImageBuffer, Pixel};
use std::{fs::File, io::BufWriter};

fn get_color_type<P>() -> Result<png::ColorType, std::io::Error>
where
    P: Pixel<Subpixel = u8>,
{
    match <P as Pixel>::CHANNEL_COUNT {
        4 => Ok(png::ColorType::Rgba),
        3 => Ok(png::ColorType::Rgb),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Incorrect color channel count / format.",
        )),
    }
}

fn get_dpi_header_data(dpi: u32) -> Vec<u8> {
    let dpm = 39.370079 * dpi as f32;
    let rounded_dpm = dpm.round() as u32;

    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(&rounded_dpm.to_be_bytes()); // Pixels per unit in X-direction.
    data.extend_from_slice(&rounded_dpm.to_be_bytes()); // Pixels per unit in Y-direction.

    data.push(1); // Indicate that meters are used as unit.
    data
}

pub fn save_png_with_dpi<P>(
    img_buffer: &ImageBuffer<P, Vec<u8>>,
    file: File,
    dpi: u32,
) -> Result<(), std::io::Error>
where
    P: Pixel<Subpixel = u8>,
{
    let w = &mut BufWriter::new(file);
    let (width, height) = img_buffer.dimensions();
    let mut encoder = png::Encoder::new(w, width, height);

    encoder.set_color(get_color_type::<P>()?);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Best);

    let data = get_dpi_header_data(dpi);
    let mut writer = encoder.write_header()?;

    writer.write_chunk(png::chunk::pHYs, data.as_slice())?;
    writer.write_image_data(img_buffer)?;
    Ok(())
}
