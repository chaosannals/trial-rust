use lopdf::dictionary;
use lopdf::{Document, Object, Stream};
use lopdf::content::{Content, Operation};

fn main() {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        // type of dictionary
        "Type" => "Font",
        // type of font, type1 is simple postscript font
        "Subtype" => "Type1",
        // basefont is postscript name of font for type1 font.
        // See PDF reference document for more details
        "BaseFont" => "Courier",
    });
    let resources_id = doc.add_object(dictionary! {
        // fonts are actually triplely nested dictionaries. Fun!
        "Font" => dictionary! {
            // F1 is the font name used when writing text.
            // It must be unique in the document. It does not
            // have to be F1
            "F1" => font_id,
        },
    });
    let content = Content {
        operations: vec![
            // BT begins a text element. it takes no operands
            Operation::new("BT", vec![]),
            // Tf specifies the font and font size. Font scaling is complicated in PDFs. Reference
            // the reference for more info.
            // The into() methods are defined based on their paired .from() methods (this
            // functionality is built into rust), and are converting the provided values into
            // An enum that represents the basic object types in PDF documents.
            Operation::new("Tf", vec!["F1".into(), 48.into()]),
            // Td adjusts the translation components of the text matrix. When used for the first
            // time after BT, it sets the initial text position on the page.
            // Note: PDF documents have Y=0 at the bottom. Thus 600 to print text near the top.
            Operation::new("Td", vec![100.into(), 600.into()]),
            // Tj prints a string literal to the page. By default, this is black text that is
            // filled in. There are other operators that can produce various textual effects and
            // colors
            Operation::new("Tj", vec![Object::string_literal("Hello World!")]),
            // ET ends the text element
            Operation::new("ET", vec![]),
        ],
    };

    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });
    let pages = dictionary! {
        // Type of dictionary
        "Type" => "Pages",
        // Vector of page IDs in document. Normally would contain more than one ID and be produced
        // using a loop of some kind
        "Kids" => vec![page_id.into()],
        // Page count
        "Count" => 1,
        // ID of resources dictionary, defined earlier
        "Resources" => resources_id,
        // a rectangle that defines the boundaries of the physical or digital media. This is the
        // "Page Size"
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);
    doc.compress();

    doc.save("example.tmp.pdf").unwrap();
}
