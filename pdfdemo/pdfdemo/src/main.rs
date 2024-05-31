use std::{
    io::Cursor,
    env::current_dir,
};
use anyhow::Result;
use image::io::Reader as ImageReader;
use pdf::{
    build::{
        PdfBuilder,
        Importer,
        PageBuilder,
        CatalogBuilder,
    },
    object::{
        InfoDict,
        ImageDict,
        ColorSpace,
        Stream,
        Resources,
        XObject,
        ImageXObject,
        Updater,
        Page,
    },
    primitive::{
        PdfString,
        Name,
    },
    enc::{
        StreamFilter,
        DCTDecodeParams,
    },
    file::FileOptions,
    content::{
        Op,
        Matrix,
        Content,
    },
};


struct Point {
    x: f32,
    y: f32
}
struct Align {
    page_rel: f32,
    page_abs: f32,
    img_rel: f32,
}

// 会有不支持的格式页，导致导出失败。

fn main() -> Result<()> {
    let here = current_dir()?;
    let pdf_old_path = here.join("test.pdf");
    let pdf_new_path = here.join("test.tmp.pdf");
    let pdf_new_path2 = here.join("test.2.tmp.pdf");

    let mut pdf_old_file = FileOptions::cached().open(pdf_old_path)?;
    let pdf_old_page = pdf_old_file.get_page(2u32)?; // 页码从 0 开始

    // test.2.tmp.pdf
    let pdf_old_resources = pdf_old_page.resources()?;
    let mut pdf_new_resources : Resources = (**pdf_old_resources).clone();

    // 图片
    let png_path = here.join("test.png");
    println!("png path: {:?}", png_path);
    let png_data = std::fs::read(png_path)?;
    let png_img = ImageReader::with_format(
        Cursor::new(&png_data),
        image::ImageFormat::Png
    ).decode()?;

    let image_dict = ImageDict {
        width: png_img.width(),
        height: png_img.height(),
        color_space: Some(ColorSpace::DeviceRGB),
        bits_per_component: Some(8),
        .. Default::default()
    };
    let image = Stream::new_with_filters(
        image_dict,
        png_data,
        vec![StreamFilter::DCTDecode(DCTDecodeParams { color_transform: None})]
    );
    let image_obj = XObject::Image(ImageXObject { inner: image });
    let image_ref = pdf_old_file.create(image_obj)?;

    let image_name = Name::from("MyImage");
    pdf_new_resources.xobjects.insert(image_name.clone(), image_ref.get_ref());

    if let Some(content) = pdf_old_page.contents.as_ref() {
        let mut ops = content.operations(&pdf_old_file.resolver())?;
        let mm = 72.0 / 25.4; // one millimeter
        // bottom right corner of the page, but 5mm margin
        let h_align = Align {
            img_rel: -1.0, // move left by image width
            page_rel: 1.0, // move right by page width
            page_abs: -5.0 * mm, // 5,mm from the right edge
        };
        let v_align = Align {
            img_rel: 0.0,
            page_rel: 0.0,
            page_abs: 5.0 * mm
        };
        let dpi = 300.;
        let px_scale = 72. / dpi;
        let media_box = pdf_old_page.media_box.unwrap();
        let scale = Point { x: png_img.width() as f32 * px_scale , y: png_img.height() as f32 * px_scale };
        let skew = Point { x: 0.0, y: 0.0 };
        let page_size = Point {
            x: media_box.right - media_box.left,
            y: media_box.top - media_box.bottom
        };
        let page_origin = Point {
            x: media_box.left,
            y: media_box.bottom
        };

        let position = Point {
            x: page_origin.x + h_align.page_abs + h_align.img_rel * scale.x + h_align.page_rel * page_size.x,
            y: page_origin.y + v_align.page_abs + v_align.img_rel * scale.y + v_align.page_rel * page_size.y
        };

        ops.append(&mut vec![
            Op::Save, // ADD IMAGE START
            Op::Transform { matrix: Matrix{ // IMAGE MANIPULATION
                a: scale.x, d: scale.y,
                b: skew.x, c: skew.y,
                e: position.x, f: position.y,
            } },
            Op::XObject {name: image_name}, // IMAGE
            Op::Restore, // ADD IMAGE STOP
        ]);
        println!("add ops to new page 2.");
        let mut page2: Page = (*pdf_old_page).clone();
        page2.contents = Some(Content::from_ops(ops));
        page2.resources = Some(pdf_old_file.create(pdf_new_resources)?.into());
        pdf_old_file.update(pdf_old_page.get_ref().get_inner(), page2)?;

        pdf_old_file.save_to(pdf_new_path2)?;
    }

    // test.tmp.pdf
    let mut builder = PdfBuilder::new(FileOptions::cached());
    let mut importer = Importer::new(pdf_old_file.resolver(), &mut builder.storage);

    let mut pdf_new_pages = Vec::new();
    let pdf_new_page = PageBuilder::clone_page(&pdf_old_page, &mut importer)?;
    pdf_new_pages.push(pdf_new_page);

    // 保存
    let catalog = CatalogBuilder::from_pages(pdf_new_pages);
    let mut info = InfoDict::default();
    info.title = Some(PdfString::from("test"));
    let data = builder.info(info).build(catalog)?;
    std::fs::write(pdf_new_path, data)?;

    Ok(())
}
