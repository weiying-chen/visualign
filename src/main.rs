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

fn sum_coors(img: &DynamicImage) -> (u32, u32) {
    let mut x_sum = 0;
    let mut y_sum = 0;

    for (x, y, pixel) in img.pixels() {
        if pixel[3] != 0 {
            x_sum += x;
            y_sum += y;
        }
    }

    (x_sum, y_sum)
}

// To-do: change this so that pixels can be moved to the left too (maybe ask stackoverflow).
// The value should be inverted (-) so the final x_avg becomes 0.
fn move_pixels_right(img: &DynamicImage, shift: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in shift..width {
            let pixel = img.get_pixel(x - shift, y);
            new_img.put_pixel(x, y, pixel);
        }
    }

    DynamicImage::ImageRgba8(new_img)
}

fn main() {
    let img = image::open("center.png").unwrap();
    let pixel_count = count_pixels(&img);
    let (x_sum, _) = sum_coors(&img);
    let x_avg = x_sum as f64 / pixel_count as f64;

    let moved_img = move_pixels_right(&img, 1);

    moved_img.save("moved.png").unwrap();

    println!("Number of non-transparent pixels: {}", pixel_count);
    println!("Average position of x: {}", x_avg);
}
