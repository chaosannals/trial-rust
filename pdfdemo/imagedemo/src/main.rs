use std::{
    io::Cursor,
    env::current_dir,
};

use anyhow::Result;
use image::io::Reader as ImageReader;

fn main() -> Result<()>{
    let here = current_dir()?;
    let png_path = here.join("test.png");
    println!("here: {:?}", png_path);

    // 路径
    let png_reader = ImageReader::open(png_path.clone())?.decode()?;
    png_reader.save("test.1.tmp.png")?;

    // 数据
    let png_data = std::fs::read(png_path)?;
    let png_reader2 = ImageReader::with_format(
        Cursor::new(&png_data),
        image::ImageFormat::Png
    ).decode()?;
    png_reader2.save("test.2.tmp.png")?;

    Ok(())
}
