use super::{InsightRepository, InsightResult};
use shared::models::{Insight, Testimonial};
use sqlx::postgres::PgRow;

pub struct PostgresInsightRepository {
    pool: sqlx::PgPool,
}

impl PostgresInsightRepository {
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
impl InsightRepository for PostgresInsightRepository {
    async fn get_testimonials_summary(
        &self,
        testimonials: Vec<Testimonial>,
    ) -> InsightResult<Vec<Insight>> {
        let model = std::env::var("OLLAMA_GEN_MODEL")
            .expect("OLLAMA_GEN_MODEL must be set in the environment");
        let assistant_prompt = "Act as a Customer Service Representative and summarize the provided feedback concisely in 20 words, highlighting key insights and areas for improvement for any business, return summary only.";
        let content_prompt = testimonials
            .iter()
            .map(|testimonial| testimonial.content.to_owned())
            .collect::<Vec<String>>()
            .join(" ");

        self.ensure_ollama_host_is_set().await.map_err(|e| e)?;

        let messages = sqlx::query_as::<_, Insight>(
            r#"
            SELECT ai.ollama_chat_complete ($1,
                    jsonb_build_array (
                        jsonb_build_object ('role',
                            'system',
                            'content',
                        $2),
                        jsonb_build_object ('role',
                            'user',
                            'content',
                        $3)
                    )                    
                ) -> 'message' ->> 'content' as message
        "#,
        )
        .bind(&model)
        .bind(&assistant_prompt)
        .bind(&content_prompt)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string());

        // map the messages to the insights
        let messages = messages?;
        let insights = messages
            .iter()
            .map(|insight| Insight {
                message: insight.message.clone(),
            })
            .collect();
        Ok(insights)
    }

    async fn get_metrics_summary(&self, metrics: Vec<String>) -> InsightResult<Vec<Insight>> {
        let model = std::env::var("OLLAMA_GEN_MODEL")
            .expect("OLLAMA_GEN_MODEL must be set in the environment");

        let assistant_prompt = "Act as a Data Analyst and summarize the provided metrics concisely in 20 words, highlighting key insights and areas for improvement for any business, return summary only.";
        let content_prompt = metrics.join(" ");

        self.ensure_ollama_host_is_set().await.map_err(|e| e)?;

        let messages = sqlx::query_as::<_, Insight>(
            r#"
            SELECT ai.ollama_chat_complete ($1,
                    jsonb_build_array (
                        jsonb_build_object ('role',
                            'system',
                            'content',
                        $2),
                        jsonb_build_object ('role',
                            'user',
                            'content',
                        $3)
                    )                    
                ) -> 'message' ->> 'content' as message
        "#,
        )
        .bind(&model)
        .bind(&assistant_prompt)
        .bind(&content_prompt)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string());

        // map the messages to the insights
        let messages = messages?;
        let insights = messages
            .iter()
            .map(|insight| Insight {
                message: insight.message.clone(),
            })
            .collect();
        Ok(insights)
    }
}
