-- ===============================
-- FlashDeal v1 Database Setup
-- ===============================
-- Enable extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- ===============================
-- Users
-- ===============================
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- ===============================
-- Products
-- ===============================
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- ===============================
-- Flash Sales
-- ===============================
CREATE TABLE flash_sales (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id),
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    total_inventory INTEGER NOT NULL CHECK (total_inventory >= 0),
    remaining_inventory INTEGER NOT NULL CHECK (remaining_inventory >= 0),
    per_user_limit INTEGER NOT NULL CHECK (per_user_limit > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- Index to support time-based reads
CREATE INDEX idx_flash_sales_time ON flash_sales (start_time, end_time);
-- ===============================
-- Orders
-- ===============================
CREATE TYPE order_status AS ENUM (
    'PENDING',
    'CONFIRMED',
    'FAILED'
);
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id BIGINT NOT NULL REFERENCES users(id),
    flash_sale_id UUID NOT NULL REFERENCES flash_sales(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    status order_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- Index to support user purchase limits
CREATE INDEX idx_orders_user_flash_sale ON orders (user_id, flash_sale_id);
-- Index to support worker polling
CREATE INDEX idx_orders_status ON orders (status);