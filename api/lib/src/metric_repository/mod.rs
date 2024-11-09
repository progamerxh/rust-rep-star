pub mod postgres_metric_repository;

pub use postgres_metric_repository::PostgresMetricRepository;

use async_trait::async_trait;
use shared::models::{CreateMetric, Metric};
use uuid::Uuid;

pub type MetricError = String;
pub type MetricResult<T> = Result<T, MetricError>;

#[async_trait]
pub trait MetricRepository: Send + Sync + 'static {
    async fn get_metrics(&self) -> MetricResult<Vec<Metric>>;
    async fn get_metric(&self, id: &Uuid) -> MetricResult<Metric>;
    async fn create_metric(&self, id: &CreateMetric) -> MetricResult<Metric>;
    async fn delete_metric(&self, id: &Uuid) -> MetricResult<Uuid>;
}
