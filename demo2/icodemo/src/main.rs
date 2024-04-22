fn main() {
    let cwd = std::env::current_dir().unwrap();
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    
    let png_path = cwd.join("demo.png");
    let png_file = std::fs::File::open(png_path).unwrap();
    let ico_image = ico::IconImage::read_png(png_file).unwrap();
    icon_dir.add_entry(ico::IconDirEntry::encode(&ico_image).unwrap());
    
    let ico_path = cwd.join("demo.ico");
    let ico_file = std::fs::File::create(ico_path).unwrap();
    icon_dir.write(ico_file).unwrap();
}
