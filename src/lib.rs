use wasm_bindgen::prelude::*;
use image::{load_from_memory, ImageOutputFormat, DynamicImage};
use image::imageops::FilterType;
use exif::{In, Reader, Tag};

#[wasm_bindgen]
pub fn resize_image(image_data: &[u8], new_width: u32, new_height: u32, output_format: Option<String>) -> Vec<u8> {
    let img = load_from_memory(image_data).expect("Failed to load image");
    let img = correct_orientation(img, image_data);
    let resized = img.resize(new_width, new_height, FilterType::Nearest);
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

fn correct_orientation(img: DynamicImage, image_data: &[u8]) -> DynamicImage {
    let exif_reader = Reader::new();
    if let Ok(exif) = exif_reader.read_raw(image_data.into()) {
        if let Some(orientation) = exif.get_field(Tag::Orientation, In::PRIMARY) {
            match orientation.value.get_uint(0) {
                Some(1) => img,
                Some(3) => img.rotate180(),
                Some(6) => img.rotate90(),
                Some(8) => img.rotate270(),
                _ => img,
            }
        } else {
            img
        }
    } else {
        img
    }
}