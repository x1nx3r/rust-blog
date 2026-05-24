use crate::{Post, PostMetadata};
use pulldown_cmark::{html, Event, Tag, TagEnd, CodeBlockKind, Options, Parser};
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::fs;
use yaml_front_matter::YamlFrontMatter;
use syntect::parsing::SyntaxSet;
use syntect::html::ClassedHTMLGenerator;
use syntect::util::LinesWithEndings;
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
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_TABLES);
                let parser = Parser::new_ext(&document.content, options);

                let ss = SyntaxSet::load_defaults_newlines();
                let mut in_code_block = false;
                let mut current_lang = String::new();
                let mut current_code = String::new();

                let events = parser.into_iter().filter_map(|event| {
                    match event {
                        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
                            in_code_block = true;
                            current_lang = lang.to_string();
                            current_code.clear();
                            None
                        }
                        Event::End(TagEnd::CodeBlock) => {
                            in_code_block = false;
                            let syntax = ss.find_syntax_by_token(&current_lang).unwrap_or_else(|| ss.find_syntax_plain_text());
                            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(syntax, &ss, syntect::html::ClassStyle::Spaced);
                            for line in LinesWithEndings::from(&current_code) {
                                html_generator.parse_html_for_line_which_includes_newline(line);
                            }
                            let highlighted = html_generator.finalize();
                            let block = format!("<pre><code class=\"language-{}\">{}</code></pre>", current_lang, highlighted);
                            Some(Event::Html(block.into()))
                        }
                        Event::Text(ref text) if in_code_block => {
                            current_code.push_str(text);
                            None
                        }
                        _ => Some(event)
                    }
                });

                let mut html_output = String::new();
                html::push_html(&mut html_output, events);

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
