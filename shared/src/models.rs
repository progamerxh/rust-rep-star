use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MetricType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Metric {
    pub time: DateTime<Utc>,
    pub metric_type_id: Uuid,
    pub value: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct CreateMetric {
    pub metric_type_id: Uuid,
    pub value: f64,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct CreateUser {
    pub email: String,
    pub name: Option<String>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Testimonial {
    pub id: Uuid,
    pub content: String,
    pub rating: f64,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct CreateTestimonial {
    pub content: String,
    pub rating: f64,
    pub user_id: Option<Uuid>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Insight {
    pub message: String,
}
