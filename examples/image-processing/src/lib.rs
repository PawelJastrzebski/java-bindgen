use java_bindgen::prelude::*;

mod resize;
mod utils;

#[derive(JLogger)]
struct Log{}

#[derive(IntoRust)]
pub struct ImgSize {
    pub widht: i32,
    pub height: i32,
}

#[java_bindgen]
pub fn resizeImage<'a>(env: &mut JNIEnv<'a> , image: Vec<u8>, input_ext: String, output_ext: String, size: ImgSize) -> JResult<Vec<u8>> {
    let log = Log::init(env);
    log.info("Read image:", env);
    let img = utils::read_image(image, &input_ext)?;
    log.info("Start resize:", env);
    let img = resize::resize(img, size.widht as u32, size.height as u32);
    log.info("Done resize:", env);
    let bytes = utils::to_bytes(img, &output_ext)?;
    log.info("To bytes", env);
    Ok(bytes)
}
