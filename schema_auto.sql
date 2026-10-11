-- TideORM Generated Schema
-- Database: Postgres
-- Generated at: 2026-09-28 20:57:40 UTC

CREATE TABLE IF NOT EXISTS "public"."_migrations" (
    "id" SERIAL,
    "version" character varying(255) NOT NULL,
    "name" character varying(255) NOT NULL,
    "applied_at" timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."categories" (
    "id" BIGSERIAL,
    "name" character varying(255) NOT NULL,
    "slug" character varying(255) NOT NULL,
    "description" text,
    "parent_id" bigint,
    "sort_order" integer DEFAULT 0,
    "created_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."category_posts" (
    "post_id" bigint NOT NULL,
    "category_id" bigint NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "public"."comments" (
    "id" BIGSERIAL,
    "post_id" bigint NOT NULL,
    "user_id" bigint NOT NULL,
    "content" text NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."customer_orders" (
    "id" BIGSERIAL,
    "order_number" text NOT NULL,
    "user_id" bigint NOT NULL,
    "status" text NOT NULL,
    "total" bigint NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."events" (
    "id" BIGSERIAL,
    "name" character varying(255) NOT NULL,
    "event_date" date NOT NULL,
    "end_date" date,
    "created_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."logs" (
    "id" BIGSERIAL,
    "message" text NOT NULL,
    "level" character varying(255) NOT NULL,
    "logged_at" timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."posts" (
    "id" BIGSERIAL,
    "author_id" bigint NOT NULL,
    "slug" character varying(255) NOT NULL,
    "status" character varying(50) NOT NULL DEFAULT 'draft'::character varying,
    "view_count" integer NOT NULL DEFAULT 0,
    "title" character varying(255) NOT NULL,
    "content" text NOT NULL,
    "excerpt" text,
    "published_at" timestamp with time zone,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now(),
    "deleted_at" timestamp with time zone,
    "translations" jsonb,
    "files" jsonb,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."products" (
    "id" BIGSERIAL,
    "name" text NOT NULL,
    "category" text NOT NULL,
    "price" bigint NOT NULL,
    "stock" integer NOT NULL,
    "active" boolean NOT NULL,
    "description" text,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."profiles" (
    "id" BIGSERIAL,
    "user_id" bigint NOT NULL,
    "bio" text,
    "website" character varying(255),
    "settings" jsonb NOT NULL DEFAULT '{}'::jsonb,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."schedules" (
    "id" BIGSERIAL,
    "name" character varying(255) NOT NULL,
    "start_time" time without time zone NOT NULL,
    "end_time" time without time zone NOT NULL,
    "created_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."sessions" (
    "id" BIGSERIAL,
    "user_id" bigint NOT NULL,
    "token" character varying(255) NOT NULL,
    "expires_at" timestamp with time zone NOT NULL,
    "last_activity_at" timestamp with time zone,
    "created_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "public"."users" (
    "id" BIGSERIAL,
    "email" character varying(255) NOT NULL,
    "name" character varying(255) NOT NULL,
    "status" character varying(50) NOT NULL DEFAULT 'active'::character varying,
    "created_at" timestamp with time zone NOT NULL DEFAULT now(),
    "updated_at" timestamp with time zone NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX IF NOT EXISTS "_migrations_version_key" ON "public"."_migrations" ("version");

CREATE UNIQUE INDEX IF NOT EXISTS "categories_slug_key" ON "public"."categories" ("slug");

CREATE UNIQUE INDEX IF NOT EXISTS "idx_post_categories_post_id_category_id_unique" ON "public"."category_posts" ("post_id", "category_id");

CREATE UNIQUE INDEX IF NOT EXISTS "idx_customer_orders_order_number_unique" ON "public"."customer_orders" ("order_number");
CREATE INDEX IF NOT EXISTS "idx_customer_orders_status" ON "public"."customer_orders" ("status");
CREATE INDEX IF NOT EXISTS "idx_customer_orders_user_id" ON "public"."customer_orders" ("user_id");

CREATE INDEX IF NOT EXISTS "idx_posts_author_id" ON "public"."posts" ("author_id");
CREATE INDEX IF NOT EXISTS "idx_posts_deleted_at" ON "public"."posts" ("deleted_at");
CREATE INDEX IF NOT EXISTS "idx_posts_slug" ON "public"."posts" ("slug");
CREATE INDEX IF NOT EXISTS "idx_posts_status" ON "public"."posts" ("status");
CREATE UNIQUE INDEX IF NOT EXISTS "posts_slug_key" ON "public"."posts" ("slug");

CREATE INDEX IF NOT EXISTS "idx_price_range" ON "public"."products" ("price", "active");
CREATE INDEX IF NOT EXISTS "idx_products_active" ON "public"."products" ("active");
CREATE INDEX IF NOT EXISTS "idx_products_category" ON "public"."products" ("category");

CREATE UNIQUE INDEX IF NOT EXISTS "profiles_user_id_key" ON "public"."profiles" ("user_id");

CREATE UNIQUE INDEX IF NOT EXISTS "sessions_token_key" ON "public"."sessions" ("token");

CREATE UNIQUE INDEX IF NOT EXISTS "users_email_key" ON "public"."users" ("email");

ALTER TABLE "public"."posts" ADD CONSTRAINT "posts_author_id_fkey" FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE;
