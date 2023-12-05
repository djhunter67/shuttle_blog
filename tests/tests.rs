use std::{collections::HashMap, sync};

use async_trait::async_trait;
use hyper::{Body, Request, StatusCode};
use shuttle_blog::{
    article_repository::{ArticleRepository, ArticleRepositoryError},
    article_routes::router,
    tables::Article,
};
use tokio::sync::RwLock;
use tower::Service;
use uuid::Uuid;

#[derive(Default)]
pub struct InMemoryArticleRepository(RwLock<HashMap<Uuid, Article>>);

#[async_trait]
impl ArticleRepository for InMemoryArticleRepository {
    async fn get_article(&self, id: Uuid) -> Result<Article, ArticleRepositoryError> {
        self.0
            .read()
            .await
            .get(&id)
            .map(ToOwned::to_owned)
            .ok_or(ArticleRepositoryError::NotFound)
    }

    async fn create_article(&self, article: Article) -> Result<Uuid, ArticleRepositoryError> {
        let uuid = Uuid::new_v4();
        self.0.write().await.insert(uuid, article);
        Ok(uuid)
    }
}
#[tokio::test]
async fn create_article() {
    let repository = InMemoryArticleRepository::default();
    let mut router = router(sync::Arc::new(repository));

    let request = Request::builder()
        .method("POST")
        .uri("/articles")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "title": "Hello",
                "content": "World",
                "published_date": "2024-01-01"
            }"#,
        ))
        .unwrap();

    let response = router.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let id: Uuid = serde_json::from_slice(&body).unwrap();

    assert!(id != Uuid::nil());
}
