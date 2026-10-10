pub mod posts;
pub mod scramble;

use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;

#[derive(Clone)]
pub struct AppState {
    pub posts: std::sync::Arc<Vec<Post>>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub posts: Vec<Post>,
    pub masthead: String,
    pub featured_scrambled: String,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub post: Post,
    pub masthead: String,
    pub title_scrambled: String,
}

/// Number of older articles served per "Fetch Earlier Editions" request.
const EDITIONS_PAGE: usize = 6;

#[derive(Template)]
#[template(path = "editions.html")]
pub struct OlderEditionsTemplate {
    pub posts: Vec<Post>,
    /// Index of the first article not yet shown; used to build the next request.
    pub next_after: usize,
    pub remaining: bool,
}

impl IntoResponse for OlderEditionsTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}



#[derive(Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    #[serde(default = "default_date")]
    pub date: String,
    #[serde(default)]
    pub formatted_date: String,
    #[serde(default = "default_author")]
    pub author: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_published")]
    pub published: bool,
}

fn default_date() -> String {
    "".to_string()
}

fn default_author() -> String {
    "Mega Nugraha".to_string()
}

fn default_published() -> bool {
    true
}

#[derive(Clone)]
pub struct Post {
    pub slug: String,
    pub metadata: PostMetadata,
    pub content: std::sync::Arc<String>, // HTML will be here
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for PostTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}




pub async fn index(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let feed_posts: Vec<Post> = state.posts.iter().filter(|p| p.slug != "about").cloned().collect();
    let featured = feed_posts.first();
    IndexTemplate {
        posts: feed_posts.clone(),
        masthead: scramble::masthead_html(),
        // Letter-level: reserved for the page's face cards. Lower
        // headline text is word-scattered client-side for performance.
        featured_scrambled: featured
            .map(|p| scramble::headline_html(&p.metadata.title))
            .unwrap_or_default(),
    }
}

pub async fn post_detail(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> impl IntoResponse {
    match state.posts.iter().find(|p| p.slug == slug) {
        Some(post) => PostTemplate {
            post: post.clone(),
            masthead: scramble::masthead_html(),
            title_scrambled: scramble::headline_html(&post.metadata.title),
        }
        .into_response(),
        None => (StatusCode::NOT_FOUND, "Post not found").into_response(),
    }
}



#[derive(Deserialize, Debug, Clone)]
pub struct EditionsQuery {
    /// Index after which to serve older editions (articles the page has not rendered yet).
    #[serde(default)]
    pub after: usize,
}

pub async fn older_editions(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<EditionsQuery>,
) -> impl IntoResponse {
    let feed: Vec<Post> = state
        .posts
        .iter()
        .filter(|p| p.slug != "about")
        .cloned()
        .collect();
    let after = query.after.min(feed.len());
    let end = (after + EDITIONS_PAGE).min(feed.len());
    let remaining = end < feed.len();

    OlderEditionsTemplate {
        posts: feed[after..end].to_vec(),
        next_after: end,
        remaining,
    }
    .into_response()
}

pub async fn about(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    match state.posts.iter().find(|p| p.slug == "about") {
        Some(post) => PostTemplate {
            post: post.clone(),
            masthead: scramble::masthead_html(),
            title_scrambled: scramble::headline_html(&post.metadata.title),
        }
        .into_response(),
        None => (StatusCode::NOT_FOUND, "About page not found").into_response(),
    }
}

pub async fn sitemap(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let mut sitemap = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    let base_url = "https://x1nx3r.dev";

    sitemap.push_str(&format!("  <url>\n    <loc>{}/</loc>\n  </url>\n", base_url));
    sitemap.push_str(&format!("  <url>\n    <loc>{}/about</loc>\n  </url>\n", base_url));

    for post in state.posts.iter() {
        if post.slug == "about" { continue; }
        sitemap.push_str(&format!("  <url>\n    <loc>{}/post/{}</loc>\n  </url>\n", base_url, post.slug));
    }

    sitemap.push_str("</urlset>");
    ([(header::CONTENT_TYPE, "application/xml")], sitemap).into_response()
}

pub async fn debug_slugs(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let slugs: Vec<String> = state.posts.iter().map(|p| p.slug.clone()).collect();
    axum::response::Json(slugs)
}

pub async fn robots_txt() -> impl IntoResponse {
    let content = "User-agent: *\nAllow: /\n\nSitemap: https://x1nx3r.dev/sitemap.xml\n";
    ([(header::CONTENT_TYPE, "text/plain")], content).into_response()
}

use axum::http::header;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

pub async fn static_handler(axum::extract::Path(file): axum::extract::Path<String>) -> impl IntoResponse {
    match Assets::get(&file) {
        Some(content) => {
            let mime = mime_guess::from_path(&file).first_or_octet_stream();
            (
                [
                    (header::CONTENT_TYPE, mime.as_ref()),
                    (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
                ],
                content.data,
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}

pub fn app() -> Router {
    let posts = posts::load_posts();
    let state = AppState { posts: std::sync::Arc::new(posts) };

    Router::new()
        .route("/assets/{*file}", get(static_handler))
        .route("/static/{*file}", get(static_handler))
        .route("/", get(index))
        .route("/robots.txt", get(robots_txt))
        .route("/about", get(about))
        .route("/editions/older", get(older_editions))
        .route("/sitemap.xml", get(sitemap))
        .route("/debug-slugs", get(debug_slugs))
        .route("/post/{slug}", get(post_detail))
        .with_state(state)
        .layer(tower_http::compression::CompressionLayer::new())
}
