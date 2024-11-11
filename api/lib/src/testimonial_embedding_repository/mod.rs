use async_trait::async_trait;
use shared::models::{Testimonial, TestimonialEmbedding};

pub use postgres_testimonial_embedding_respository::PostgresTestimonialEmbeddingRepository;
mod postgres_testimonial_embedding_respository;

pub type TestimonialEmbeddingError = String;
pub type TestimonialEmbeddingResult<T> = Result<T, TestimonialEmbeddingError>;

#[async_trait]
pub trait TestimonialEmbeddingRepository: Send + Sync + 'static {
    async fn create_testimonial_embedding(
        &self,
        testimonial: Testimonial,
    ) -> TestimonialEmbeddingResult<TestimonialEmbedding>;
    async fn get_testimonial_embeddings(
        &self,
        query: String,
    ) -> TestimonialEmbeddingResult<Vec<TestimonialEmbedding>>;
    // async fn get_testimonial_embedding(
    //     &self,
    //     id: &Uuid,
    // ) -> TestimonialEmbeddingResult<TestimonialEmbedding>;
    // async fn update_testimonial_embedding(
    //     &self,
    //     id: &TestimonialEmbedding,
    // ) -> TestimonialEmbeddingResult<TestimonialEmbedding>;
    // async fn delete_testimonial_embedding(&self, id: &Uuid) -> TestimonialEmbeddingResult<Uuid>;
}
