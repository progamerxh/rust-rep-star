
CREATE TABLE IF NOT EXISTS testimonial_embedding  (
    id uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT testimonial_embedding_pkey PRIMARY KEY,
    document_id uuid CONSTRAINT testimonial_embedding_document_id_fkey REFERENCES testimonials(id),
    metadata JSONB,
    contents TEXT,
    embedding VECTOR(1536)
)