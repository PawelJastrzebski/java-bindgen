use std::io::Cursor;
use image::{io::Reader as ImageReader, DynamicImage, ImageResult};

pub fn read_image(bytes: Vec<u8>, ext: &str) -> ImageResult<DynamicImage> {
    let mut img = ImageReader::new(Cursor::new(bytes));
    let format = image::ImageFormat::from_extension(ext).unwrap_or(image::ImageFormat::Jpeg);
    img.set_format(format);
    img.decode()
}

pub fn to_bytes(img2: DynamicImage, ext: &str) -> ImageResult<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    let format = image::ImageFormat::from_extension(ext).unwrap_or(image::ImageFormat::Jpeg);
    img2.write_to(&mut Cursor::new(&mut bytes), format)?;
    Ok(bytes)
}