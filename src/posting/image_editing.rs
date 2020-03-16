use tokio::try_join;
use image::{ImageResult, DynamicImage};
use image::imageops::overlay;

const BACKGROUND: &[u8] = include_bytes!("background.png");

async fn load_background_image() -> ImageResult<DynamicImage> {
    image::load_from_memory_with_format(
        BACKGROUND,
        image::ImageFormat::Png
    )
}

async fn load_foreground_image(data: Vec<u8>) -> ImageResult<DynamicImage> {
    image::load_from_memory_with_format(
        &data,
        image::ImageFormat::Png
    )
}

pub(crate) async fn add_background(data: Vec<u8>) -> Vec<u8> {
    let (mut bg, fg) = try_join!(
        load_background_image(),
        load_foreground_image(data)
    ).unwrap();

    overlay(&mut bg, &fg, 716, 490);

    let mut output = vec![];
    bg.write_to(&mut output, image::ImageFormat::Png).unwrap();
    output
}
