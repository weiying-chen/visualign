use image::{DynamicImage, GenericImageView, ImageBuffer};

pub struct ImgProcessor {
    img: DynamicImage,
}

impl ImgProcessor {
    pub fn new(img: DynamicImage) -> Self {
        Self { img }
    }

    pub fn count_pixels(&self) -> u32 {
        let mut count = 0;

        for (_, _, pixel) in self.img.pixels() {
            let alpha = pixel[3];

            if alpha != 0 {
                count += 1;
            }
        }

        count
    }

    pub fn sum_coors(&self) -> (u64, u64) {
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

    pub fn move_pixels(&self, shift: i32) -> DynamicImage {
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

    pub fn get_image_center(&self) -> (u32, u32) {
        let (width, height) = self.img.dimensions();
        let x_center = width / 2;
        let y_center = height / 2;

        (x_center, y_center)
    }
}
