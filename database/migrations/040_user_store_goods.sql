CREATE TABLE IF NOT EXISTS user_store_goods (
    user_id INTEGER NOT NULL,
    goods_id INTEGER NOT NULL,
    buy_count INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, goods_id)
);

CREATE INDEX IF NOT EXISTS idx_user_store_goods_user ON user_store_goods(user_id);
