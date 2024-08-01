use image::{imageops, DynamicImage};

pub fn resize(image: DynamicImage, width: usize, height: usize) -> DynamicImage {
    image.resize(width as u32, height as u32, imageops::FilterType::Nearest)
}