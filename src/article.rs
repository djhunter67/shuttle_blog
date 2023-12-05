use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{article_error::AppError, article_repository::ArticleRepository, tables::Article};

pub async fn get_article<T: ArticleRepository>(
    Path(id): Path<Uuid>,
    State(repository): State<Arc<T>>,
) -> Result<Json<Article>, AppError> {
    let article = repository.get_article(id).await?;
    Ok(Json(article))
}

pub async fn create_article<T: ArticleRepository>(
    State(repository): State<Arc<T>>,
    Json(article): Json<Article>,
) -> Result<Json<Uuid>, AppError> {
    let id = repository.create_article(article).await?;
    Ok(Json(id))
}
