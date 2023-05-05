use image::imageops;
use std::fs;
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
    // Read the contents of the input folder
    let input_dir = "./input";
    let entries = fs::read_dir(input_dir).unwrap();
    let text_img = image::open("text.png").unwrap();

    // Iterate over the directory entries
    for entry in entries {
        let path = entry.unwrap().path();

        // Only process image files
        if let Some("png") = path.extension().and_then(|s| s.to_str()) {
            // Load the input image and text image
            let input_img = image::open(&path).unwrap();

            // Apply the operation to the input image
            let mut shifted_img = visual_center(input_img);
            imageops::overlay(&mut shifted_img, &text_img, 0, 0);

            // Save the result to an output file
            let output_path = format!("./output/{}", path.file_name().unwrap().to_str().unwrap());
            shifted_img.save(&output_path).unwrap();
        }
    }
}
