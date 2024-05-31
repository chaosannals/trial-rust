use std::env::current_dir;
use anyhow::Result;
use image::io::Reader as ImageReader;

fn main() -> Result<()> {
    let here = current_dir()?;
    let png_path = here.join("test.png");
    println!("here: {:?}", png_path);
    let img = ImageReader::open(png_path)?.decode()?;
    img.save("test.tmp.png")?;
    Ok(())
}
