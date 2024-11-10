use super::{InsightRepository, InsightResult};
use shared::models::{Insight, Testimonial};

pub struct PostgresInsightRepository {
    pool: sqlx::PgPool,
}

impl PostgresInsightRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl InsightRepository for PostgresInsightRepository {
    async fn get_insights(&self, testimonials: Vec<Testimonial>) -> InsightResult<Vec<Insight>> {
        let model = "llama3.2";
        let assistant_prompt = "Act as a Customer Service Representative and summarize the provided feedback concisely in 20 words, highlighting key insights and areas for improvement for any business";
        let content_prompt = testimonials
            .iter()
            .map(|testimonial| testimonial.content.to_owned())
            .collect::<Vec<String>>()
            .join(" ");

        // Ensure Ollama host is set
        sqlx::query("SELECT set_config('ai.ollama_host', 'https://97d6-2405-4802-803b-9b20-a565-4d6c-bcea-d6a0.ngrok-free.app', false);")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

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
