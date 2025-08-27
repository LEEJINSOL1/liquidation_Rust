CREATE TABLE exchanges (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE NOT NULL
);

CREATE TABLE markets (
  id SERIAL PRIMARY KEY,
  symbol TEXT NOT NULL,
  exchange_id INT NOT NULL REFERENCES exchanges(id),
  UNIQUE (exchange_id, symbol)
);

CREATE TABLE trades (
  id BIGSERIAL PRIMARY KEY,
  ts TIMESTAMPTZ NOT NULL,
  market_id INT NOT NULL REFERENCES markets(id),
  price DOUBLE PRECISION NOT NULL,
  qty DOUBLE PRECISION NOT NULL,
  side TEXT NOT NULL,
  buyer_addr TEXT,
  seller_addr TEXT
);

CREATE TABLE tracked_wallets (
  addr TEXT PRIMARY KEY,
  first_seen TIMESTAMPTZ NOT NULL DEFAULT now(),
  labels TEXT[] DEFAULT '{}',
  note TEXT
);

CREATE TABLE positions (
  id BIGSERIAL PRIMARY KEY,
  addr TEXT NOT NULL REFERENCES tracked_wallets(addr),
  market_id INT NOT NULL REFERENCES markets(id),
  ts TIMESTAMPTZ NOT NULL,
  entry_price DOUBLE PRECISION,
  liq_price DOUBLE PRECISION,
  take_profit DOUBLE PRECISION,
  leverage DOUBLE PRECISION,
  size DOUBLE PRECISION,
  wallet_balance_usd DOUBLE PRECISION
);

CREATE INDEX ON positions (market_id, ts);
CREATE INDEX ON positions (addr);

CREATE TABLE pnl_agg (
  id BIGSERIAL PRIMARY KEY,
  addr TEXT NOT NULL REFERENCES tracked_wallets(addr),
  market_id INT NOT NULL REFERENCES markets(id),
  bucket DATE NOT NULL,
  realized DOUBLE PRECISION,
  unrealized DOUBLE PRECISION,
  win_rate DOUBLE PRECISION,
  sharpe DOUBLE PRECISION
);

CREATE UNIQUE INDEX ON pnl_agg (addr, market_id, bucket);

CREATE TABLE liq_heatmap (
  id BIGSERIAL PRIMARY KEY,
  market_id INT NOT NULL REFERENCES markets(id),
  bucket_start DOUBLE PRECISION NOT NULL,
  bucket_end DOUBLE PRECISION NOT NULL,
  count BIGINT NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX ON liq_heatmap (market_id, bucket_start, bucket_end);

CREATE TABLE spread_snapshots (
  id BIGSERIAL PRIMARY KEY,
  ts TIMESTAMPTZ NOT NULL,
  base_market_id INT NOT NULL REFERENCES markets(id),
  ref_market_id INT NOT NULL REFERENCES markets(id),
  base_price DOUBLE PRECISION NOT NULL,
  ref_price DOUBLE PRECISION NOT NULL,
  spread_bp DOUBLE PRECISION NOT NULL
);

CREATE INDEX ON spread_snapshots (ts);

CREATE TABLE stable_metrics (
  id BIGSERIAL PRIMARY KEY,
  ts TIMESTAMPTZ NOT NULL,
  stable_symbol TEXT NOT NULL,
  net_mint_burn DOUBLE PRECISION,
  exchange_netflow_usd DOUBLE PRECISION
);

CREATE TABLE alert_rules (
  id SERIAL PRIMARY KEY,
  rule_type TEXT NOT NULL,
  params JSONB NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE alert_logs (
  id BIGSERIAL PRIMARY KEY,
  rule_id INT REFERENCES alert_rules(id),
  ts TIMESTAMPTZ NOT NULL DEFAULT now(),
  payload JSONB
);
