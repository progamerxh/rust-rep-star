CREATE TABLE IF NOT EXISTS  "public"."testimonial_embeddings" (
    id SERIAL PRIMARY KEY,
    testimonial_id uuid,
    testimonial_content TEXT NOT NULL,
    embedding VECTOR NOT NULL
);
