-- TideORM Generated Schema
-- Database: Postgres
-- Generated at: 2026-03-28 13:18:11 UTC

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

CREATE INDEX IF NOT EXISTS "idx_posts_user_id" ON "public"."posts" ("user_id");
CREATE INDEX IF NOT EXISTS "idx_posts_status" ON "public"."posts" ("status");
CREATE INDEX IF NOT EXISTS "idx_posts_metadata" ON "public"."posts" ("metadata");
CREATE INDEX IF NOT EXISTS "idx_posts_tags" ON "public"."posts" ("tags");

CREATE INDEX IF NOT EXISTS "idx_products_attributes" ON "public"."products" ("attributes");
CREATE INDEX IF NOT EXISTS "idx_products_category" ON "public"."products" ("category");

CREATE UNIQUE INDEX IF NOT EXISTS "profiles_user_id_key" ON "public"."profiles" ("user_id");

CREATE UNIQUE INDEX IF NOT EXISTS "users_email_key" ON "public"."users" ("email");

