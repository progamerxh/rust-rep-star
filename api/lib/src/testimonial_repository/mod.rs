use shared::models::{CreateTestimonial, Testimonial};
use uuid::Uuid;
pub type TestimonialError = String;
pub type TestimonialResult<T> = Result<T, TestimonialError>;

#[async_trait::async_trait]
pub trait TestimonialRepository: Send + Sync + 'static {
    async fn get_testimonials(&self) -> TestimonialResult<Vec<Testimonial>>;
    async fn get_testimonial(&self, id: &Uuid) -> TestimonialResult<Testimonial>;
    async fn create_testimonial(&self, id: &CreateTestimonial) -> TestimonialResult<Testimonial>;
    async fn update_testimonial(&self, id: &Testimonial) -> TestimonialResult<Testimonial>;
    async fn delete_testimonial(&self, id: &Uuid) -> TestimonialResult<Uuid>;
}
