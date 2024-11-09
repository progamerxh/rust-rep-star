use super::{InsightRepository, InsightResult};
use shared::models::Insight;

pub struct PostgresInsightRepository {
    pool: sqlx::PgPool,
}

impl PostgresInsightRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl InsightRepository for PostgresInsightRepository {
    async fn get_insights(&self) -> InsightResult<Vec<Insight>> {
        sqlx::query_as::<_, Insight>(
            r#"
      SELECT id, email, name, created_at, updated_at
      FROM insights
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
