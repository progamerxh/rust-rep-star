use async_trait::async_trait;
use shared::models::Insight;

mod postgres_insight_repository;
pub use postgres_insight_repository::PostgresInsightRepository;
pub type InsightError = String;
pub type InsightResult<T> = Result<T, InsightError>;

#[async_trait]
pub trait InsightRepository: Send + Sync + 'static {
    async fn get_insights(&self) -> InsightResult<Vec<Insight>>;
}
