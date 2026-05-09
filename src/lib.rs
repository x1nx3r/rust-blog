use axum::{
    routing::get,
    Router,
    response::{IntoResponse, Html},
    http::StatusCode,
};
use tower_http::services::ServeDir;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "clicked.html")]
pub struct ClickedTemplate;

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            ).into_response(),
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
            ).into_response(),
        }
    }
}

pub async fn index() -> impl IntoResponse {
    IndexTemplate
}

pub async fn clicked() -> impl IntoResponse {
    ClickedTemplate
}

pub async fn reset() -> impl IntoResponse {
    Html(r##"
        <div id="dynamic-content">
            <button hx-get="/clicked" hx-target="#dynamic-content" hx-swap="outerHTML">
                Experience the Speed
            </button>
        </div>
    "##)
}

pub fn app() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .route("/clicked", get(clicked))
        .route("/reset", get(reset))
}
