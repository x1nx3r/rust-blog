pub mod posts;

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
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub post: Post,
}

#[derive(Template)]
#[template(path = "clicked.html")]
pub struct ClickedTemplate;

#[derive(Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    #[serde(default = "default_date")]
    pub date: String,
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
    pub content: String, // HTML will be here
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

impl IntoResponse for ClickedTemplate {
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

#[derive(Template)]
#[template(path = "wire.html")]
pub struct WireTemplate {
    pub posts: Vec<Post>,
}

impl IntoResponse for WireTemplate {
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
    IndexTemplate {
        posts: state.posts.clone(),
    }
}

pub async fn post_detail(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> impl IntoResponse {
    match state.posts.iter().find(|p| p.slug == slug) {
        Some(post) => PostTemplate { post: post.clone() }.into_response(),
        None => (StatusCode::NOT_FOUND, "Post not found").into_response(),
    }
}

pub async fn wire_posts(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    WireTemplate {
        posts: state.posts.clone(),
    }
}

pub async fn clicked() -> impl IntoResponse {
    ClickedTemplate
}

pub async fn reset() -> impl IntoResponse {
    Html(
        r##"
        <div id="dynamic-content">
            <button hx-get="/clicked" hx-target="#dynamic-content" hx-swap="outerHTML">
                Experience the Speed
            </button>
        </div>
    "##,
    )
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
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}

pub fn app() -> Router {
    let posts = posts::load_posts();
    let state = AppState { posts };

    Router::new()
        .route("/assets/{*file}", get(static_handler))
        .route("/static/{*file}", get(static_handler))
        .route("/", get(index))
        .route("/post/{slug}", get(post_detail))
        .route("/api/wire-posts", get(wire_posts))
        .route("/clicked", get(clicked))
        .route("/reset", get(reset))
        .with_state(state)
}
