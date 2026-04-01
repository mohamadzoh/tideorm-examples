-- TideORM Generated Schema
-- Database: Postgres
-- Generated at: 2026-04-01 15:00:28 UTC

CREATE TABLE IF NOT EXISTS "public"."_migrations" (
    "id" BIGSERIAL,
    "version" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "applied_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."comments" (
    "id" BIGSERIAL,
    "post_id" BIGINT NOT NULL,
    "user_id" BIGINT NOT NULL,
    "content" TEXT NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."customer_orders" (
    "id" BIGINT,
    "order_number" TEXT NOT NULL,
    "user_id" BIGINT NOT NULL,
    "status" TEXT NOT NULL,
    "total" BIGINT NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."events" (
    "id" BIGSERIAL,
    "name" CHARACTER VARYING NOT NULL,
    "event_date" DATE NOT NULL,
    "end_date" DATE,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."logs" (
    "id" BIGSERIAL,
    "message" TEXT NOT NULL,
    "level" CHARACTER VARYING NOT NULL,
    "logged_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."posts" (
    "id" BIGSERIAL,
    "user_id" BIGINT NOT NULL,
    "title" CHARACTER VARYING NOT NULL,
    "content" TEXT NOT NULL,
    "status" CHARACTER VARYING NOT NULL DEFAULT 'draft'::character varying,
    "tags" ARRAY NOT NULL DEFAULT '{}'::text[],
    "metadata" JSONB NOT NULL DEFAULT '{}'::jsonb,
    "view_count" INTEGER NOT NULL DEFAULT 0,
    "published_at" TIMESTAMP WITH TIME ZONE,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "deleted_at" TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."products" (
    "id" BIGSERIAL,
    "name" CHARACTER VARYING NOT NULL,
    "category" CHARACTER VARYING NOT NULL,
    "price" BIGINT NOT NULL,
    "stock" INTEGER NOT NULL DEFAULT 0,
    "active" BOOLEAN NOT NULL DEFAULT true,
    "attributes" JSONB NOT NULL DEFAULT '{}'::jsonb,
    "related_skus" ARRAY NOT NULL DEFAULT '{}'::text[],
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."profiles" (
    "id" BIGSERIAL,
    "user_id" BIGINT NOT NULL,
    "bio" TEXT,
    "website" CHARACTER VARYING,
    "settings" JSONB NOT NULL DEFAULT '{}'::jsonb,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."schedules" (
    "id" BIGSERIAL,
    "name" CHARACTER VARYING NOT NULL,
    "start_time" TIME WITHOUT TIME ZONE NOT NULL,
    "end_time" TIME WITHOUT TIME ZONE NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."sessions" (
    "id" BIGSERIAL,
    "user_id" BIGINT NOT NULL,
    "token" CHARACTER VARYING NOT NULL,
    "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "last_activity_at" TIMESTAMP WITH TIME ZONE,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."tokenization_orders" (
    "id" BIGSERIAL,
    "user_id" BIGINT NOT NULL,
    "product_id" BIGINT NOT NULL,
    "quantity" INTEGER NOT NULL,
    "total_cents" INTEGER NOT NULL,
    "status" CHARACTER VARYING NOT NULL DEFAULT 'pending'::character varying,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."tokenization_products" (
    "id" BIGSERIAL,
    "sku" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "price" INTEGER NOT NULL,
    "stock" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."tokenization_users" (
    "id" BIGSERIAL,
    "email" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "status" CHARACTER VARYING NOT NULL DEFAULT 'active'::character varying,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "deleted_at" TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."users" (
    "id" BIGSERIAL,
    "email" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "status" CHARACTER VARYING NOT NULL DEFAULT 'active'::character varying,
    "password_hash" CHARACTER VARYING,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX IF NOT EXISTS "_migrations_version_key" ON "public"."_migrations" ("version");

CREATE INDEX IF NOT EXISTS "idx_posts_user_id" ON "public"."posts" ("user_id");
CREATE INDEX IF NOT EXISTS "idx_posts_status" ON "public"."posts" ("status");
CREATE INDEX IF NOT EXISTS "idx_posts_metadata" ON "public"."posts" ("metadata");
CREATE INDEX IF NOT EXISTS "idx_posts_tags" ON "public"."posts" ("tags");

CREATE INDEX IF NOT EXISTS "idx_products_category" ON "public"."products" ("category");
CREATE INDEX IF NOT EXISTS "idx_products_attributes" ON "public"."products" ("attributes");

CREATE UNIQUE INDEX IF NOT EXISTS "profiles_user_id_key" ON "public"."profiles" ("user_id");

CREATE UNIQUE INDEX IF NOT EXISTS "sessions_token_key" ON "public"."sessions" ("token");

CREATE INDEX IF NOT EXISTS "idx_tokenization_orders_status" ON "public"."tokenization_orders" ("status");
CREATE INDEX IF NOT EXISTS "idx_tokenization_orders_user_id" ON "public"."tokenization_orders" ("user_id");
CREATE INDEX IF NOT EXISTS "idx_tokenization_orders_product_id" ON "public"."tokenization_orders" ("product_id");

CREATE UNIQUE INDEX IF NOT EXISTS "tokenization_products_sku_key" ON "public"."tokenization_products" ("sku");

CREATE UNIQUE INDEX IF NOT EXISTS "tokenization_users_email_key" ON "public"."tokenization_users" ("email");

CREATE UNIQUE INDEX IF NOT EXISTS "users_email_key" ON "public"."users" ("email");

