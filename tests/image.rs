use std::env;
use webfetch_server::error::Result;
use webfetch_server::image::{self, ImageFormat};

#[test]
#[ignore]
fn test_image_resize_and_split() -> Result<()> {
    let input_path = env::var("WEBFETCH_TEST_IMAGE")
        .unwrap_or_else(|_| panic!("missing env var: WEBFETCH_TEST_IMAGE"));
    let format = ImageFormat::Webp;
    let suffix = format.suffix();
    let _app = image::init_lib(10).expect("init_lib failed");

    let input = std::fs::read(input_path)?;
    let input = image::resize(&input, 600, format)?;

    for (i, chunk) in image::split_by_height(&input, 600, format)?.enumerate() {
        let output = env::temp_dir().join(format!("sample_{i}{suffix}"));
        std::fs::write(output, chunk?)?;
    }

    Ok(())
}
