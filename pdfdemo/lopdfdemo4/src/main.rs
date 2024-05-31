use lopdf::Document;
use anyhow::Result;

// 版本 0.32.0 没有发布 async futures
#[tokio::main]
async fn main() -> Result<()> {
    let mut doc = Document::load("test.pdf")?;
    // await 需要 async future
    // let mut doc = Document::load("test.pdf").await?;
    
    doc.version = "1.4".to_string();
    let _ = doc.replace_text(1, "Hello World!", "Modified text!");
    // Store file in current working directory.
    // Note: Line is excluded when running tests
    doc.save("modified.async.tmp.pdf")?;
    Ok(())
}
