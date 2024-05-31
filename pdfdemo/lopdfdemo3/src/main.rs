use lopdf::Document;
use anyhow::Result;

// 无效。
fn main() -> Result<()> {
    let mut doc = Document::load("merged.tmp.pdf")?;

    doc.version = "1.4".to_string();
    let _ = doc.replace_text(1, "Hello", "Modified text!!!!!!!!");
    // Store file in current working directory.
    // Note: Line is excluded when running tests
    
    doc.save("modified.sync.tmp.pdf")?;
    Ok(())
}
