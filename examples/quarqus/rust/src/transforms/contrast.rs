use image::{DynamicImage};

pub fn contrast(image: DynamicImage, value: f32) -> DynamicImage {
    image.adjust_contrast(value)
}