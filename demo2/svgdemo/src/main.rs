use std::path::Path;
use std::convert::AsRef;

fn scale_to_png(svg_path: impl AsRef<Path>,target: &str,w: u32,h: u32) {
    let mut svg_opt = usvg::Options::default();
    let fontdb = usvg::fontdb::Database::new();
    let svg_data = std::fs::read(svg_path).unwrap();
    let tree = usvg::Tree::from_data(&svg_data, &svg_opt, &fontdb).unwrap();
    let pixmap_size = tree.size().to_int_size();
    let width = pixmap_size.width();
    let height = pixmap_size.height();

    let mut pixmap = tiny_skia::Pixmap::new(w, h).unwrap();
    resvg::render(&tree, tiny_skia::Transform::from_scale((w as f32) / (width as f32), (h as f32) / (height as f32)), &mut pixmap.as_mut());
    pixmap.save_png(target).unwrap();
}

fn main() {
    let cwd = std::env::current_dir().unwrap();
    let exe_path = std::env::current_exe().unwrap();
    let svg_path = cwd.join("demo.svg");
    println!("start at: {cwd:?}");
    println!("exe at: {exe_path:?}");
    println!("svg at: {svg_path:?}");

    let mut svg_opt = usvg::Options::default();
    let fontdb = usvg::fontdb::Database::new();
    let svg_data = std::fs::read(svg_path.clone()).unwrap();
    let tree = usvg::Tree::from_data(&svg_data, &svg_opt, &fontdb).unwrap();
    let pixmap_size = tree.size().to_int_size();
    let width = pixmap_size.width();
    let height = pixmap_size.height();
    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap.save_png("aaaa.png").unwrap();

    let mut pixmap32x32 = tiny_skia::Pixmap::new(32, 32).unwrap();
    resvg::render(&tree, tiny_skia::Transform::from_scale(32f32 / (width as f32), 32f32 / (height as f32)), &mut pixmap32x32.as_mut());
    pixmap32x32.save_png("32x32.png").unwrap();

    scale_to_png(svg_path, "128x128.png", 128, 128);
}
