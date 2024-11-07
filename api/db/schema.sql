CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "public"."metric_types" (
    "id" uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT metric_types_pkey PRIMARY KEY,
    "name" text NOT NULL,
    "description" text,
    "createdAt" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" timestamp(3) NOT NULL
);

CREATE TABLE IF NOT EXISTS "public"."metrics" (
    "id" uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT metrics_pkey PRIMARY KEY,
    "time" timestamp(3) NOT NULL,
    "metricTypeId" text NOT NULL,
    "value" float8 NOT NULL,
    "createdAt" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "public"."testimonials" (
    "id" uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT testimonials_pkey PRIMARY KEY,
    "content" text NOT NULL,
    "rating" float8 NOT NULL,
    "userId" text,
    "createdAt" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "public"."users" (
    "id" uuid DEFAULT uuid_generate_v4() NOT NULL CONSTRAINT users_pkey PRIMARY KEY,
    "email" text NOT NULL,
    "name" text,
    "createdAt" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS users_email_key ON public.users USING btree (email);
