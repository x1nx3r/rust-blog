use crate::{Post, PostMetadata};
use pulldown_cmark::{Parser, html};
use std::fs;
use yaml_front_matter::YamlFrontMatter;

pub fn load_posts() -> Vec<Post> {
    let mut posts = Vec::new();
    let paths = fs::read_dir("contents").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().map_or(false, |ext| ext == "md") {
            let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
            let raw_content = fs::read_to_string(&path).unwrap();

            if let Ok(document) = YamlFrontMatter::parse::<PostMetadata>(&raw_content) {
                if document.metadata.published {
                    let parser = Parser::new(&document.content);
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);

                    let mut metadata = document.metadata;
                    if metadata.date.is_empty() {
                        metadata.date = "2026-05-24".to_string();
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
                        content: html_output,
                    });
                }
            } else {
                eprintln!("Warning: failed to parse frontmatter for post at {:?}", path);
            }
        }
    }

    posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));
    posts
}
