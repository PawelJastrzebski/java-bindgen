use image::{imageops, DynamicImage};

pub fn resize(image: DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, imageops::FilterType::Nearest)
}