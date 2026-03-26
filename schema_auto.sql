-- TideORM Generated Schema
-- Database: Postgres
-- Generated at: 2026-03-26 15:13:39 UTC

CREATE TABLE IF NOT EXISTS ""public"."posts"" (
    "id" BIGSERIAL,
    "user_id" BIGINT NOT NULL,
    "title" TEXT NOT NULL,
    "body" TEXT NOT NULL,
    "published" BOOLEAN NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS ""public"."users"" (
    "id" BIGSERIAL,
    "email" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "bio" TEXT,
    "active" BOOLEAN NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "_migrations" (
    "id" BIGSERIAL,
    "version" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "applied_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "bench_products" (
    "id" BIGSERIAL,
    "name" CHARACTER VARYING NOT NULL,
    "category" CHARACTER VARYING NOT NULL,
    "price" INTEGER NOT NULL,
    "stock" INTEGER NOT NULL,
    "active" BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "bench_users" (
    "id" BIGSERIAL,
    "email" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "age" INTEGER NOT NULL,
    "active" BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "callback_users" (
    "id" BIGSERIAL,
    "email" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "categories" (
    "id" BIGSERIAL,
    "name" CHARACTER VARYING NOT NULL,
    "slug" CHARACTER VARYING NOT NULL,
    "description" TEXT,
    "parent_id" BIGINT,
    "sort_order" INTEGER DEFAULT 0,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "comments" (
    "id" BIGSERIAL,
    "post_id" BIGINT NOT NULL,
    "user_id" BIGINT NOT NULL,
    "parent_id" BIGINT,
    "body" TEXT NOT NULL,
    "approved" BOOLEAN DEFAULT false,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "customer_orders" (
    "id" BIGINT,
    "order_number" TEXT NOT NULL,
    "user_id" BIGINT NOT NULL,
    "status" TEXT NOT NULL,
    "total" BIGINT NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "events" (
    "id" BIGSERIAL,
    "name" CHARACTER VARYING NOT NULL,
    "event_date" DATE NOT NULL,
    "end_date" DATE,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "logs" (
    "id" BIGSERIAL,
    "message" TEXT NOT NULL,
    "level" CHARACTER VARYING NOT NULL,
    "logged_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "or_bench_users" (
    "id" BIGSERIAL,
    "name" CHARACTER VARYING NOT NULL,
    "email" CHARACTER VARYING NOT NULL,
    "status" CHARACTER VARYING NOT NULL,
    "role" CHARACTER VARYING NOT NULL,
    "department" CHARACTER VARYING NOT NULL,
    "age" INTEGER NOT NULL,
    "active" BOOLEAN NOT NULL DEFAULT true,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "post_categories" (
    "post_id" BIGINT NOT NULL,
    "category_id" BIGINT NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "posts" (
    "id" BIGSERIAL,
    "user_id" BIGINT NOT NULL,
    "title" CHARACTER VARYING NOT NULL,
    "slug" CHARACTER VARYING NOT NULL,
    "content" TEXT,
    "excerpt" TEXT,
    "published" BOOLEAN DEFAULT false,
    "published_at" TIMESTAMP WITH TIME ZONE,
    "metadata" JSONB,
    "tags" ARRAY,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMP WITH TIME ZONE,
    "search_vector" TSVECTOR,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "products" (
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

CREATE TABLE IF NOT EXISTS "profiles" (
    "id" BIGSERIAL,
    "user_id" BIGINT NOT NULL,
    "bio" TEXT,
    "website" CHARACTER VARYING,
    "settings" JSONB NOT NULL DEFAULT '{}'::jsonb,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "schedules" (
    "id" BIGSERIAL,
    "name" CHARACTER VARYING NOT NULL,
    "start_time" TIME WITHOUT TIME ZONE NOT NULL,
    "end_time" TIME WITHOUT TIME ZONE NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "sessions" (
    "id" BIGSERIAL,
    "user_id" BIGINT NOT NULL,
    "token" CHARACTER VARYING NOT NULL,
    "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "last_activity_at" TIMESTAMP WITH TIME ZONE,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "test_raw_json_types" (
    "id" BIGSERIAL,
    "enabled" BOOLEAN NOT NULL,
    "payload" JSONB NOT NULL,
    "amount" NUMERIC NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "uuid_value" UUID NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "timestamp_users" (
    "id" BIGSERIAL,
    "email" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "login_count" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "tokenization_orders" (
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

CREATE TABLE IF NOT EXISTS "tokenization_products" (
    "id" BIGSERIAL,
    "sku" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "price" INTEGER NOT NULL,
    "stock" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "tokenization_users" (
    "id" BIGSERIAL,
    "email" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "status" CHARACTER VARYING NOT NULL DEFAULT 'active'::character varying,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    "deleted_at" TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "users" (
    "id" BIGSERIAL,
    "email" CHARACTER VARYING NOT NULL,
    "name" CHARACTER VARYING NOT NULL,
    "password_hash" CHARACTER VARYING NOT NULL,
    "active" BOOLEAN NOT NULL DEFAULT true,
    "role" CHARACTER VARYING NOT NULL DEFAULT 'user'::character varying,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMP WITH TIME ZONE,
    "avatar_url" CHARACTER VARYING,
    "bio" TEXT,
    "website" CHARACTER VARYING,
    "location" CHARACTER VARYING,
    "settings" JSONB DEFAULT '{}'::jsonb,
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX IF NOT EXISTS "_migrations_version_key" ON "_migrations" ("version");

CREATE INDEX IF NOT EXISTS "idx_bench_products_price" ON "bench_products" ("price");
CREATE INDEX IF NOT EXISTS "idx_bench_products_active" ON "bench_products" ("active");
CREATE INDEX IF NOT EXISTS "idx_bench_products_category" ON "bench_products" ("category");

CREATE UNIQUE INDEX IF NOT EXISTS "categories_slug_key" ON "categories" ("slug");

CREATE INDEX IF NOT EXISTS "idx_comments_user_id" ON "comments" ("user_id");
CREATE INDEX IF NOT EXISTS "idx_comments_post_id_approved" ON "comments" ("post_id", "approved");

CREATE INDEX IF NOT EXISTS "idx_or_bench_age" ON "or_bench_users" ("age");
CREATE INDEX IF NOT EXISTS "idx_or_bench_role" ON "or_bench_users" ("role");
CREATE INDEX IF NOT EXISTS "idx_or_bench_department" ON "or_bench_users" ("department");
CREATE INDEX IF NOT EXISTS "idx_or_bench_active" ON "or_bench_users" ("active");
CREATE INDEX IF NOT EXISTS "idx_or_bench_status" ON "or_bench_users" ("status");

CREATE UNIQUE INDEX IF NOT EXISTS "idx_post_categories_post_id_category_id_unique" ON "post_categories" ("post_id", "category_id");

CREATE INDEX IF NOT EXISTS "idx_posts_published" ON "posts" ("published");
CREATE INDEX IF NOT EXISTS "idx_posts_user_id" ON "posts" ("user_id");
CREATE INDEX IF NOT EXISTS "idx_posts_published_at" ON "posts" ("published_at");
CREATE UNIQUE INDEX IF NOT EXISTS "posts_slug_key" ON "posts" ("slug");
CREATE INDEX IF NOT EXISTS "idx_posts_search" ON "posts" ("search_vector");

CREATE INDEX IF NOT EXISTS "idx_products_category" ON "products" ("category");
CREATE INDEX IF NOT EXISTS "idx_products_attributes" ON "products" ("attributes");

CREATE UNIQUE INDEX IF NOT EXISTS "profiles_user_id_key" ON "profiles" ("user_id");

CREATE UNIQUE INDEX IF NOT EXISTS "sessions_token_key" ON "sessions" ("token");

CREATE UNIQUE INDEX IF NOT EXISTS "timestamp_users_email_key" ON "timestamp_users" ("email");

CREATE INDEX IF NOT EXISTS "idx_tokenization_orders_status" ON "tokenization_orders" ("status");
CREATE INDEX IF NOT EXISTS "idx_tokenization_orders_user_id" ON "tokenization_orders" ("user_id");
CREATE INDEX IF NOT EXISTS "idx_tokenization_orders_product_id" ON "tokenization_orders" ("product_id");

CREATE UNIQUE INDEX IF NOT EXISTS "tokenization_products_sku_key" ON "tokenization_products" ("sku");

CREATE UNIQUE INDEX IF NOT EXISTS "tokenization_users_email_key" ON "tokenization_users" ("email");

CREATE INDEX IF NOT EXISTS "idx_users_active" ON "users" ("active");
CREATE UNIQUE INDEX IF NOT EXISTS "users_email_key" ON "users" ("email");
CREATE INDEX IF NOT EXISTS "idx_users_role" ON "users" ("role");

