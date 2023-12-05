use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::article::{create_article, get_article};

use crate::article_repository::ArticleRepository;

pub fn router<T: ArticleRepository>(repository: Arc<T>) -> Router {
    Router::new()
        .route("/articles/:id", get(get_article))
        .route("/articles", post(create_article))
        .with_state(repository)
}
