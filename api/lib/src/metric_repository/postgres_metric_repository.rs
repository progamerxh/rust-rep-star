use super::{MetricRepository, MetricResult};
use shared::models::{CreateMetric, Metric};

pub struct PostgresMetricRepository {
    pool: sqlx::PgPool,
}

impl PostgresMetricRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MetricRepository for PostgresMetricRepository {
    async fn get_metrics(&self) -> MetricResult<Vec<Metric>> {
        sqlx::query_as::<_, Metric>(
            r#"
      SELECT time, value, metric_type_id, created_at, updated_at
      FROM metrics
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_metric(&self, metric_id: &uuid::Uuid) -> MetricResult<Metric> {
        sqlx::query_as::<_, Metric>(
            r#"
      SELECT time, value, metric_type_id, created_at, updated_at
      FROM metrics
      WHERE id = $1
      "#,
        )
        .bind(metric_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_metric(&self, create_metric: &CreateMetric) -> MetricResult<Metric> {
        sqlx::query_as::<_, Metric>(
            r#"
      INSERT INTO metrics (value, metric_type_id)
      VALUES ($1, $2)
      RETURNING time, value, metric_type_id, created_at, updated_at
      "#,
        )
        .bind(&create_metric.value)
        .bind(&create_metric.metric_type_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_metric(&self, metric_id: &uuid::Uuid) -> MetricResult<uuid::Uuid> {
        sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
      DELETE FROM metrics
      WHERE id = $1
      RETURNING id
      "#,
        )
        .bind(&metric_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
