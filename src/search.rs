use tantivy::IndexReader;
use tantivy::IndexWriter;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tempfile::TempDir;
use anyhow::Result;

fn get_schema() -> Result<Schema, anyhow::Error> {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    Ok(schema)
}

struct SearchIndex {
    index: Index,
    index_writer: IndexWriter,
    reader: IndexReader,
    title: Field,
    body: Field,
}

fn get_indexer(schema: &Schema) -> Result<SearchIndex, anyhow::Error> {
    let index_path = TempDir::new()?;
    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let index_writer = index.writer(50_000_000)?;
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    Ok(SearchIndex {
        index,
        index_writer,
        reader,
        title,
        body
    })
}

fn create_document_one(schema: &Schema, index_writer: &mut IndexWriter) -> Result<(), anyhow::Error> {
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let mut old_man_doc = Document::default();
    old_man_doc.add_text(title, "The Old Man and the Sea");
    old_man_doc.add_text(
        body,
        "He was an old man who fished alone in a skiff in the Gulf Stream and \
         he had gone eighty-four days now without taking a fish.",
    );
    index_writer.add_document(old_man_doc).unwrap();
    index_writer.commit()?;
    Ok(())
}

fn create_document_two(schema: &Schema, index_writer: &mut IndexWriter) -> Result<(), anyhow::Error> {
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let mut old_man_doc = Document::default();
    index_writer.add_document(doc!(
        title => "Of Mice and Men",
        body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
                bank and runs deep and green. The water is warm too, for it has slipped twinkling \
                over the yellow sands in the sunlight before reaching the narrow pool. On one \
                side of the river the golden foothill slopes curve up to the strong and rocky \
                Gabilan Mountains, but on the valley side the water is lined with trees—willows \
                fresh and green with every spring, carrying in their lower leaf junctures the \
                debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
                limbs and branches that arch over the pool"
        ));
    index_writer.add_document(doc!(
    title => "Of Mice and Men",
    body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
            bank and runs deep and green. The water is warm too, for it has slipped twinkling \
            over the yellow sands in the sunlight before reaching the narrow pool. On one \
            side of the river the golden foothill slopes curve up to the strong and rocky \
            Gabilan Mountains, but on the valley side the water is lined with trees—willows \
            fresh and green with every spring, carrying in their lower leaf junctures the \
            debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
            limbs and branches that arch over the pool"
    ));

    index_writer.add_document(doc!(
        title => "Frankenstein",
        title => "The Modern Prometheus",
        body => "You will rejoice to hear that no disaster has accompanied the commencement of an \
                 enterprise which you have regarded with such evil forebodings.  I arrived here \
                 yesterday, and my first task is to assure my dear sister of my welfare and \
                 increasing confidence in the success of my undertaking."
        ));
    
    index_writer.commit()?;
    Ok(())
    
}

fn search(schema: &Schema, reader: &IndexReader, index: &Index, title: &Field, body: &Field) -> Result<(), anyhow::Error> {

    
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![*title, *body]);
    let query = query_parser.parse_query("sea whale")?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }
    Ok(())
}

fn do_something() -> tantivy::Result<()> {
    let schema = get_schema().unwrap();
    let SearchIndex { index, mut index_writer, reader, title, body } = get_indexer(&schema).unwrap();
    
    create_document_one(&schema, &mut index_writer);

    Ok(())
}