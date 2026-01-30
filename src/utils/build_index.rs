use jieba_rs::Jieba;
use tantivy::{doc, Index, IndexWriter, TantivyDocument};
use tantivy::schema::{Schema, STORED, STRING, TEXT};
use crate::model::game::Game;

pub fn build_game_index(game_list: Vec<Game>)->tantivy::Result<Index>{
    let jie_ba = Jieba::new();
    let mut schema_builder = Schema::builder();

    let id = schema_builder.add_i64_field("id", STORED);
    let name = schema_builder.add_text_field("name",TEXT | STORED);
    let url = schema_builder.add_text_field("url", STORED);
    let pan_type = schema_builder.add_text_field("pan_type", STRING | STORED);

    let schema = schema_builder.build();
    let index = Index::create_in_ram(schema);
    let mut writer: IndexWriter<TantivyDocument> = index.writer(50_000_000)?;

    for game in game_list{
        let tokenizer_name = jie_ba.cut(game.name.as_str(), false).join(" ");
        writer.add_document(doc!(
            id => game.id as i64,
            name => tokenizer_name,
            url => game.url,
            pan_type => format!("{:?}", game.pan_type)
        ))?;
    }
    writer.commit()?;

    Ok(index)
}