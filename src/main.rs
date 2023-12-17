use std::env;
use std::path::PathBuf;
use visualign::img_processor::ImgProcessor;
use visualign::img_util;

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
    // Change this to the dir containing your images.
    const DIR: &str = "/home/alex/Desktop";
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut root_dir = PathBuf::from(manifest_dir);

    root_dir.push("text.png");

    let entries = std::fs::read_dir(DIR).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();

        if let Some("png") = path.extension().and_then(|s| s.to_str()) {
            let input_img = image::open(&path).unwrap();
            let shifted_img = visual_center(input_img);
            let filename = path.file_stem().unwrap().to_string_lossy();
            let output_path = format!("{}/{}_centered.png", DIR, filename);
            let file = std::fs::File::create(&output_path).unwrap();
            const DPI: u32 = 300;

            img_util::save_png_with_dpi(&shifted_img.into_rgba8(), file, DPI).unwrap();
        }
    }
}
