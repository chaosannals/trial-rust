use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::tokenizer::NgramTokenizer;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy};
// use tempfile::TempDir;
use anyhow::Result;
use std::{fs, env::current_dir};

fn main() -> Result<()> {
    // let index_dir = TempDir::new()?;
    let here_dir = current_dir()?;
    let index_dir = here_dir.join("index.tdb");

    let (index, schema) = if !index_dir.exists() {
        fs::create_dir_all(&index_dir)?;

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT);
    
        // 自定义索引字段
        let text_field_indexing = TextFieldIndexing::default()
            .set_tokenizer("ngram3")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_field_indexing)
            .set_stored();
        schema_builder.add_text_field("number", text_options);
    
    
        let schema = schema_builder.build();
        (Index::create_in_dir(&index_dir, schema.clone())?, schema)
    } else {
        let index = Index::open_in_dir(&index_dir)?;
        let schema = index.schema();
        (index, schema)
    };


    // 注册自定义索引分词器
    index
        .tokenizers()
        .register("ngram3", NgramTokenizer::new(3, 3, false).unwrap());

    let mut index_writer: IndexWriter = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let number = schema.get_field("number").unwrap();

    let mut old_man_doc = TantivyDocument::default();
    old_man_doc.add_text(title, "The Old Man and the Sea");
    old_man_doc.add_text(
        body,
        "He was an old man who fished alone in a skiff in the Gulf Stream and he had gone \
         eighty-four days now without taking a fish.",
    );
    old_man_doc.add_text(number, "6578412345");
    index_writer.add_document(old_man_doc)?;

    index_writer.add_document(doc!(
        title => "Of Mice and Men",
        body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
                bank and runs deep and green. The water is warm too, for it has slipped twinkling \
                over the yellow sands in the sunlight before reaching the narrow pool. On one \
                side of the river the golden foothill slopes curve up to the strong and rocky \
                Gabilan Mountains, but on the valley side the water is lined with trees—willows \
                fresh and green with every spring, carrying in their lower leaf junctures the \
                debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
                limbs and branches that arch over the pool",
        number => "1234567"
        ))?;
    index_writer.commit()?;

    // 
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![title, body, number]);
    let query = query_parser.parse_query("sea whale 578")?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}", retrieved_doc.to_json(&schema));
    }

    Ok(())
}
