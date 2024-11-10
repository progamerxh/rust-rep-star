use std::str::FromStr;

use async_trait::async_trait;
use shared::models::{CreateTestimonial, Testimonial};
use uuid::Uuid;

pub use postgres_testimonial_repository::PostgresTestimonialRepository;
mod postgres_testimonial_repository;

pub type TestimonialError = String;
pub type TestimonialResult<T> = Result<T, TestimonialError>;

#[derive(Debug, PartialEq)]
pub enum TimeDuration {
    LastDay,
    LastWeek,
    LastMonth,
    LastYear,
}

impl FromStr for TimeDuration {
    type Err = ();

    fn from_str(input: &str) -> Result<TimeDuration, Self::Err> {
        match input {
            "day" => Ok(TimeDuration::LastDay),
            "week" => Ok(TimeDuration::LastWeek),
            "month" => Ok(TimeDuration::LastMonth),
            "year" => Ok(TimeDuration::LastYear),
            _ => Err(()),
        }
    }
}

#[async_trait]
pub trait TestimonialRepository: Send + Sync + 'static {
    async fn get_testimonials(&self) -> TestimonialResult<Vec<Testimonial>>;
    async fn get_testimonials_by_time_duration(
        &self,
        time_duration: TimeDuration,
    ) -> TestimonialResult<Vec<Testimonial>>;
    async fn get_testimonial(&self, id: &Uuid) -> TestimonialResult<Testimonial>;
    async fn create_testimonial(&self, id: &CreateTestimonial) -> TestimonialResult<Testimonial>;
    async fn update_testimonial(&self, id: &Testimonial) -> TestimonialResult<Testimonial>;
    async fn delete_testimonial(&self, id: &Uuid) -> TestimonialResult<Uuid>;
}

// Embedding traits
#[async_trait]
pub trait TestimonialEmbeddingRepository: Send + Sync + 'static {
    async fn embed_and_write(&self, id: &Testimonial) -> TestimonialResult<Testimonial>;
}
