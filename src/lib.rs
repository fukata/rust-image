use wasm_bindgen::prelude::*;
use image::{load_from_memory, ImageOutputFormat, GenericImageView};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn resize_image(image_data: &[u8], new_width: u32, new_height: u32, output_format: Option<String>) -> Vec<u8> {
    let img = load_from_memory(image_data).expect("Failed to load image");
    let resized = img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
    let mut buffer = Vec::new();

    let format = match output_format {
        Some(fmt) => match fmt.as_str() {
            "png" => ImageOutputFormat::Png,
            "jpeg" => ImageOutputFormat::Jpeg(80),
            "jpg" => ImageOutputFormat::Jpeg(80),
            "gif" => ImageOutputFormat::Gif,
            _ => ImageOutputFormat::Png, // default to PNG if unknown format
        },
        None => {
            // Guess the format from the input data
            match image::guess_format(image_data).expect("Failed to guess image format") {
                image::ImageFormat::Png => ImageOutputFormat::Png,
                image::ImageFormat::Jpeg => ImageOutputFormat::Jpeg(80),
                image::ImageFormat::Gif => ImageOutputFormat::Gif,
                _ => ImageOutputFormat::Png, // default to PNG if unknown format
            }
        }
    };

    resized.write_to(&mut buffer, format).expect("Failed to write image");
    buffer
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
pub fn get_image_info(image_data: &[u8]) -> ImageInfo {
    let img = load_from_memory(image_data).expect("Failed to load image");
    let (width, height) = img.dimensions();
    ImageInfo { width, height }
}