use crate::{Post, PostMetadata};
use pulldown_cmark::{Parser, html};
use rust_embed::RustEmbed;
use yaml_front_matter::YamlFrontMatter;
use chrono::NaiveDate;

#[derive(RustEmbed)]
#[folder = "contents/"]
struct EmbeddedPosts;

pub fn load_posts() -> Vec<Post> {
    let mut posts = Vec::new();

    for file_path in EmbeddedPosts::iter() {
        if !file_path.ends_with(".md") {
            continue;
        }
        
        let slug = std::path::Path::new(file_path.as_ref())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let embedded_file = EmbeddedPosts::get(file_path.as_ref()).unwrap();
        let raw_content = std::str::from_utf8(&embedded_file.data).unwrap();

        if let Ok(document) = YamlFrontMatter::parse::<PostMetadata>(raw_content) {
            if document.metadata.published {
                let parser = Parser::new(&document.content);
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);

                let mut metadata = document.metadata;
                if metadata.date.is_empty() {
                    metadata.date = "2026-05-24".to_string();
                }
                
                if let Ok(parsed_date) = NaiveDate::parse_from_str(&metadata.date, "%Y-%m-%d") {
                    metadata.formatted_date = parsed_date.format("%B %-d, %Y").to_string();
                } else {
                    metadata.formatted_date = metadata.date.clone();
                }

                if metadata.summary.is_empty() {
                    let words: Vec<&str> = document.content.split_whitespace().take(30).collect();
                    let mut generated = words.join(" ");
                    if document.content.split_whitespace().count() > 30 {
                        generated.push_str("...");
                    }
                    metadata.summary = generated;
                }

                posts.push(Post {
                    slug,
                    metadata,
                    content: std::sync::Arc::new(html_output),
                });
            }
        } else {
            eprintln!("Warning: failed to parse frontmatter for post at {}", file_path);
        }
    }

    // Sort by raw YYYY-MM-DD
    posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));
    posts
}
