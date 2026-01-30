use std::vec::Vec;
use tantivy::collector::TopDocs;
use tantivy::{Index, TantivyDocument};
use tantivy::query::QueryParser;
use tantivy::schema::Value;
use tracing::{debug, info};
use crate::model::game::{Game, PanType};
use jieba_rs::Jieba;

pub fn search_game(index: &Index, keyword: &str) -> anyhow::Result<Vec<Game>> {
    info!("search game: {}", keyword);
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();

    let name = schema.get_field("name")?;
    let query_parser = QueryParser::for_index(index, vec![name]);
    
    // 使用jieba分词处理搜索关键词，以匹配索引中的分词
    let jie_ba = Jieba::new();
    let tokenized_keyword = jie_ba.cut(keyword, false).join(" ");
    info!("tokenized search keyword: {}", tokenized_keyword);

    let top_docs = searcher.search(&query_parser.parse_query(&tokenized_keyword)?, &TopDocs::with_limit(10))?;

    let mut result: Vec<Game> = vec![];

    let id_f = schema.get_field("id")?;
    let name_f = schema.get_field("name")?;
    let url_f = schema.get_field("url")?;
    let pan_type_f = schema.get_field("pan_type")?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

        let id = retrieved_doc
            .get_first(id_f)
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let name = retrieved_doc
            .get_first(name_f)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let url = retrieved_doc
            .get_first(url_f)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let pan_type_str = retrieved_doc
            .get_first(pan_type_f)
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let pan_type = match pan_type_str {
            "XunLei" => PanType::XunLei,
            "Baidu" => PanType::Baidu,
            "Quark" => PanType::Quark,
            _ => PanType::Other,
        };

        let game = Game::new(
            id as i32,
            name,
            url,
            pan_type,
        );
        result.push(game);
    }
    info!("search game result size: {}", result.len());
    debug!("search game result: {:?}", result);
    Ok(result)
}