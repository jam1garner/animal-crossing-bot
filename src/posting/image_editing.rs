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

async fn load_foreground_image<T: AsRef<[u8]>>(data: T) -> ImageResult<DynamicImage> {
    image::load_from_memory_with_format(
        data.as_ref(),
        image::ImageFormat::Png
    )
}

const POSITIONS: &[&[(u32, u32)]] = &[
    &[],
    &[(716, 490)],
    &[(630, 490), (890, 490)],
];

pub(crate) async fn add_background(data: Vec<Vec<u8>>) -> Vec<u8> {
    let mut bg = load_background_image().await.unwrap();
    let fgs = match &data[..] {
        &[] => vec![],
        &[ref fg] => vec![load_foreground_image(fg).await.unwrap()],
        &[ref fg1, ref fg2] => vec![
            load_foreground_image(fg1).await.unwrap(),
            load_foreground_image(fg2).await.unwrap(),
        ],
        _ => vec![]
    };

    let num_imgs = fgs.len();
    for (fg, &pos) in fgs.into_iter().zip(POSITIONS[num_imgs].iter()) {
        overlay(&mut bg, &fg, pos.0, pos.1);
    }

    let mut output = vec![];
    bg.write_to(&mut output, image::ImageFormat::Png).unwrap();
    output
}
