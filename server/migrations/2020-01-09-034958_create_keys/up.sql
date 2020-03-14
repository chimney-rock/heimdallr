CREATE TABLE keys (
  id SERIAL PRIMARY KEY,
  "primary" uuid NOT NULL DEFAULT gen_random_uuid(),
  "secondary" uuid NOT NULL DEFAULT gen_random_uuid(),
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_keys_primary ON keys USING btree("primary");
CREATE INDEX idx_keys_secondary ON keys USING btree("secondary");
SELECT diesel_manage_updated_at('keys');
