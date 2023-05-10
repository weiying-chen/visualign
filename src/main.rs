use image::imageops;
use visual_center::img_processor::ImgProcessor;
use visual_center::img_util;

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

fn extract_name<'a>(array: &'a [&'a str], string: &'a str) -> Option<&'a str> {
    for name in array {
        if string.contains(name) {
            return Some(name);
        }
    }
    None
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
        "french_bulldog",
        "german_shepherd",
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

            let output_path = match extracted_name {
                Some(name) => format!("./output/{}.png", name),
                None => {
                    println!("None returned for file: {:?}", path);
                    continue;
                }
            };

            let file = std::fs::File::create(&output_path).unwrap();

            img_util::save_png_with_dpi(&shifted_img.into_rgba8(), file, 300).unwrap();
        }
    }
}
