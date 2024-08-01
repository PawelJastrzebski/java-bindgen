use image::DynamicImage;
use crate::transforms::contrast::contrast;
use crate::transforms::resize::resize;

mod resize;
mod contrast;

pub fn extract_args(def: &str, prefix: &str, args_size: usize) -> Option<Vec<String>> {
    let def = def.trim();
    if !def.starts_with(prefix) {
        return None;
    }

    let Some((_, args)) = def.split_once(prefix) else {
        return None;
    };

    let args: Vec<&str> = args.split(",").collect();
    if args.len() == args_size {
        Some(args.into_iter().map(|v| v.trim()).map(ToString::to_string).collect())
    } else {
        None
    }
}

trait ExtractArg {
    fn get_usize(&self, index: usize) -> Option<usize>;
    fn get_f32(&self, index: usize) -> Option<f32>;
}

impl ExtractArg for Vec<String> {
    fn get_usize(&self, index: usize) -> Option<usize> {
        self.get(index)?.parse().ok()
    }
    fn get_f32(&self, index: usize) -> Option<f32> {
        self.get(index)?.parse().ok()
    }
}

pub fn process(image: DynamicImage, transforms: Vec<String>, applied_transforms: &mut Vec<String>) -> DynamicImage {
    let mut result = image;
    for def in transforms {
        if let Some(args) = extract_args(&def, "resize:", 2) {
            let Some(width) = args.get_usize(0) else { continue };
            let Some(height) = args.get_usize(1) else { continue };
            result = resize(result, width, height);
            applied_transforms.push(def);
            continue;
        }

        if let Some(args) = extract_args(&def, "contrast:", 1) {
            let Some(value) = args.get_f32(0) else { continue };
            result = contrast(result, value);
            applied_transforms.push(def);
            continue;
        }
    }

    result
}



