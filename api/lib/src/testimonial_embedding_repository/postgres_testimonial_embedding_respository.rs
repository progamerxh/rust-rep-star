use super::{TestimonialEmbeddingRepository, TestimonialEmbeddingResult};
use shared::models::{Testimonial, TestimonialEmbedding};
use sqlx::postgres::PgRow;

pub struct PostgresTestimonialEmbeddingRepository {
    pool: sqlx::PgPool,
}

impl PostgresTestimonialEmbeddingRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    async fn ensure_ollama_host_is_set(&self) -> Result<Vec<PgRow>, String> {
        let ollama_host =
            std::env::var("OLLAMA_HOST").expect("OLLAMA_HOST must be set in the environment");

        sqlx::query(r#"SELECT set_config('ai.ollama_host', $1, false);"#)
            .bind(&ollama_host)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl TestimonialEmbeddingRepository for PostgresTestimonialEmbeddingRepository {
    async fn create_testimonial_embedding(
        &self,
        testimonial: Testimonial,
    ) -> TestimonialEmbeddingResult<TestimonialEmbedding> {
        let model = std::env::var("OLLAMA_EMBED_MODEL")
            .expect("OLLAMA_EMBED_MODEL must be set in the environment");

        self.ensure_ollama_host_is_set().await.map_err(|e| e)?;

        sqlx::query_as(
            r#"
        INSERT INTO testimonial_embeddings (testimonial_id, testimonial_content, embedding)
        VALUES ($1, $2, (SELECT ollama_embed FROM ai.ollama_embed($3, $2)))
        RETURNING id, testimonial_id, testimonial_content
        "#,
        )
        .bind(&testimonial.id)
        .bind(&testimonial.content)
        .bind(&model)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_testimonial_embeddings(
        &self,
        query: String,
    ) -> TestimonialEmbeddingResult<Vec<TestimonialEmbedding>> {
        let model = std::env::var("OLLAMA_EMBED_MODEL")
            .expect("OLLAMA_EMBED_MODEL must be set in the environment");

        self.ensure_ollama_host_is_set().await.map_err(|e| e)?;

        sqlx::query_as(
            r#"
        SELECT id, testimonial_id, testimonial_content
        FROM testimonial_embeddings
        ORDER BY embedding <-> (SELECT ollama_embed FROM ai.ollama_embed($1, $2))
        LIMIT 10
        "#,
        )
        .bind(&model)
        .bind(&query)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
