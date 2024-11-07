use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct MetricType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Metric {
    pub time: DateTime<Utc>,
    pub metric_type_id: Uuid,
    pub value: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Testimonial {
    pub id: Uuid,
    pub content: String,
    pub rating: f64,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateTestimonial {
    pub content: String,
    pub rating: f64,
    pub user_id: Option<Uuid>,
}
