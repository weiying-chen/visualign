use image::{DynamicImage, GenericImageView, ImageBuffer};

fn count_pixels(img: &DynamicImage) -> u32 {
    let mut count = 0;

    for (_, _, pixel) in img.pixels() {
        if pixel[3] != 0 {
            count += 1;
        }
    }

    count
}

fn sum_coors(img: &DynamicImage) -> (u64, u64) {
    let mut x_sum = 0u64;
    let mut y_sum = 0u64;

    for (x, y, pixel) in img.pixels() {
        if pixel[3] != 0 {
            x_sum += x as u64;
            y_sum += y as u64;
        }
    }

    (x_sum, y_sum)
}

fn move_pixels(img: &DynamicImage, shift: i32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::new(width, height);

    for y in 0..height {
        let left = i32::max(0, shift);
        let right = i32::min(0, shift) + width as i32;

        for x in left..right {
            let pixel = img.get_pixel((x - shift) as u32, y);

            new_img.put_pixel(x as u32, y, pixel);
        }
    }

    DynamicImage::ImageRgba8(new_img)
}

fn get_image_center(img: &DynamicImage) -> (u32, u32) {
    let (width, height) = img.dimensions();
    let x_center = width / 2;
    let y_center = height / 2;

    (x_center, y_center)
}

fn main() {
    let img = image::open("input.png").unwrap();
    let pixel_count = count_pixels(&img);
    let (x_sum, _) = sum_coors(&img);
    let (x_center, _) = get_image_center(&img);
    let x_avg = x_sum as f64 / pixel_count as f64;
    let shift = x_center as f64 - x_avg;

    // let shifted_img = move_pixels(&img, -shift as i32);
    let shifted_img = move_pixels(&img, shift.round() as i32);

    shifted_img.save("output.png").unwrap();

    println!("pixel_count: {}", pixel_count);
    println!("x_center: {}", x_center);
    println!("x_avg: {}", x_avg);
    println!("shift: {}", shift.round() as i32);
}
