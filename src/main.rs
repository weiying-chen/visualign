use image::{DynamicImage, GenericImageView, ImageBuffer};

struct ImgProcessor {
    img: DynamicImage,
}

impl ImgProcessor {
    fn new(img: DynamicImage) -> Self {
        Self { img }
    }

    fn count_pixels(&self) -> u32 {
        let mut count = 0;

        for (_, _, pixel) in self.img.pixels() {
            if pixel[3] != 0 {
                count += 1;
            }
        }

        count
    }

    fn sum_coors(&self) -> (u64, u64) {
        let mut x_sum = 0u64;
        let mut y_sum = 0u64;

        for (x, y, pixel) in self.img.pixels() {
            if pixel[3] != 0 {
                x_sum += x as u64;
                y_sum += y as u64;
            }
        }

        (x_sum, y_sum)
    }

    fn move_pixels(&self, shift: i32) -> DynamicImage {
        let (width, height) = self.img.dimensions();
        let mut new_img = ImageBuffer::new(width, height);

        for y in 0..height {
            let left = i32::max(0, shift);
            let right = i32::min(0, shift) + width as i32;

            for x in left..right {
                let pixel = self.img.get_pixel((x - shift) as u32, y);

                new_img.put_pixel(x as u32, y, pixel);
            }
        }

        DynamicImage::ImageRgba8(new_img)
    }

    fn get_image_center(&self) -> (u32, u32) {
        let (width, height) = self.img.dimensions();
        let x_center = width / 2;
        let y_center = height / 2;

        (x_center, y_center)
    }
}

fn main() {
    let img = image::open("input.png").unwrap();
    let img_processor = ImgProcessor::new(img);
    let pixel_count = img_processor.count_pixels();
    let (x_sum, _) = img_processor.sum_coors();
    let (x_center, _) = img_processor.get_image_center();
    let x_avg = x_sum as f64 / pixel_count as f64;
    let shift = x_center as f64 - x_avg;
    let shifted_img = img_processor.move_pixels(shift.round() as i32);

    shifted_img.save("output.png").unwrap();

    println!("pixel_count: {}", pixel_count);
    println!("x_center: {}", x_center);
    println!("x_avg: {}", x_avg);
    println!("shift: {}", shift.round() as i32);
}
