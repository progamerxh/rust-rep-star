CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "public"."metric_types" (
    "id" uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT metric_types_pkey PRIMARY KEY,
    "name" text NOT NULL,
    "description" text,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS "public"."metrics" (
    "time" TIMESTAMPTZ NOT NULL,
    "metric_type_id" text NOT NULL,
    "value" DOUBLE PRECISION NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "public"."testimonials" (
    "id" uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT testimonials_pkey PRIMARY KEY,
    "content" text NOT NULL,
    "rating" DOUBLE PRECISION NOT NULL,
    "user_id" text,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "public"."users" (
    "id" uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT users_pkey PRIMARY KEY,
    "email" text NOT NULL,
    "name" text,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS users_email_key ON public.users USING btree (email);
CREATE INDEX IF NOT EXISTS metrics_time_key ON public.metrics USING btree (time);