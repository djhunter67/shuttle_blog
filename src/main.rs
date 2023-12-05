use std::sync::Arc;

use axum::Router;
use sqlx::{Executor, PgPool};

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../db/init.sql"))
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    // this line will change.
    let router = Router::new().with_state(Arc::new(pool));

    Ok(router.into())
}

