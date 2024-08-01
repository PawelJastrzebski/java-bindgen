mod utils;
mod transforms;

use java_bindgen::prelude::*;

#[derive(Default, JavaClass)]
pub struct ImageInput {
    pub image: Vec<u8>,
    pub ext: String,
}

#[allow(non_snake_case)]
#[derive(Default, JavaClass)]
pub struct TransformResult {
    pub image: Vec<u8>,
    pub ext: String,
    pub appliedTransforms: JList<String>
}

#[java_bindgen]
pub fn processImage(input: ImageInput, transforms: JList<String>) -> JResult<TransformResult> {
    let mut applied_transforms = Vec::new();
    let image = utils::read_image(input.image, &input.ext)?;
    let result = transforms::process(image, transforms.0, &mut applied_transforms);

    Ok(TransformResult {
        image: utils::to_bytes(result, &input.ext)?,
        ext: input.ext,
        appliedTransforms: JList(applied_transforms)
    })
}