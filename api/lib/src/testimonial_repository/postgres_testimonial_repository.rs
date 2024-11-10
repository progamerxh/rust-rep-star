use super::{
    TestimonialEmbeddingRepository, TestimonialRepository, TestimonialResult, TimeDuration,
};
use shared::models::{CreateTestimonial, Testimonial};

pub struct PostgresTestimonialRepository {
    pool: sqlx::PgPool,
}

impl PostgresTestimonialRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TestimonialRepository for PostgresTestimonialRepository {
    async fn get_testimonials(&self) -> TestimonialResult<Vec<Testimonial>> {
        sqlx::query_as::<_, Testimonial>(
            r#"
      SELECT id, content, rating, user_id, created_at, updated_at
      FROM testimonials
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_testimonials_by_time_duration(
        &self,
        time_duration: TimeDuration,
    ) -> TestimonialResult<Vec<Testimonial>> {
        // deinfe Postgresql query based on time_duration utilizing built-in INTERVAL
        let interval = match time_duration {
            TimeDuration::LastDay => "1 day",
            TimeDuration::LastWeek => "1 week",
            TimeDuration::LastMonth => "1 month",
            TimeDuration::LastYear => "1 year",
        };

        sqlx::query_as::<_, Testimonial>(
            r#"
      SELECT id, content, rating, user_id, created_at, updated_at
      FROM testimonials
      WHERE NOW() - created_at < $1::INTERVAL;
      "#,
        )
        .bind(&interval)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_testimonial(&self, testimonial_id: &uuid::Uuid) -> TestimonialResult<Testimonial> {
        sqlx::query_as::<_, Testimonial>(
            r#"
      SELECT id, content, rating, user_id, created_at, updated_at
      FROM testimonials
      WHERE id = $1
      "#,
        )
        .bind(testimonial_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_testimonial(
        &self,
        create_testimonial: &CreateTestimonial,
    ) -> TestimonialResult<Testimonial> {
        sqlx::query_as::<_, Testimonial>(
            r#"
      INSERT INTO testimonials (content, rating, user_id)
      VALUES ($1, $2, $3)
      RETURNING id, content, rating, user_id, created_at, updated_at
      "#,
        )
        .bind(&create_testimonial.content)
        .bind(&create_testimonial.rating)
        .bind(&create_testimonial.user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_testimonial(
        &self,
        testimonial: &Testimonial,
    ) -> TestimonialResult<Testimonial> {
        sqlx::query_as::<_, Testimonial>(
            r#"
      UPDATE testimonials
      SET content = $2, rating = $3, user_id = $4
      WHERE id = $1
      RETURNING id, title, director, year, poster, created_at, updated_at
      "#,
        )
        .bind(&testimonial.id)
        .bind(&testimonial.content)
        .bind(&testimonial.rating)
        .bind(&testimonial.user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_testimonial(
        &self,
        testimonial_id: &uuid::Uuid,
    ) -> TestimonialResult<uuid::Uuid> {
        sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
      DELETE FROM testimonials
      WHERE id = $1
      RETURNING id
      "#,
        )
        .bind(&testimonial_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl TestimonialEmbeddingRepository for PostgresTestimonialRepository {
    async fn embed_and_write(&self, testimonial: &Testimonial) -> TestimonialResult<Testimonial> {
        sqlx::query_as::<_, Testimonial>(
            r#"
      INSERT INTO testimonials (content, rating, user_id)
      VALUES ($1, $2, $3)
      RETURNING id, content, rating, user_id, created_at, updated_at
      "#,
        )
        .bind(&testimonial.content)
        .bind(&testimonial.rating)
        .bind(&testimonial.user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
