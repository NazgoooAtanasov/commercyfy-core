CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE products (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY, 
    product_name VARCHAR NOT NULL,
    product_description VARCHAR NOT NULL,
    product_color VARCHAR
);

CREATE TABLE images (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    src VARCHAR NOT NULL,
    srcset VARCHAR NOT NULL,
    alt VARCHAR,

    product_id uuid,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE TABLE categories (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    category_name VARCHAR NOT NULL,
    category_description VARCHAR,

    category_reference VARCHAR NOT NULL,
    UNIQUE (category_reference)
);

CREATE TABLE categories_products (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,

    category_id uuid,
    product_id uuid,
    FOREIGN KEY (category_id) references categories(id),
    FOREIGN KEY (product_id) references products(id)
);

CREATE TYPE portaluserroles AS ENUM (
    'READER',
    'EDITOR',
    'ADMIN'
);
CREATE TABLE portal_users (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    roles portaluserroles[]
);

CREATE TABLE inventories (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    inventory_name VARCHAR NOT NULL UNIQUE,
    inventory_reference VARCHAR NOT NULL UNIQUE
);

CREATE TABLE inventories_products (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,

    allocation INT NOT NULL DEFAULT 0,

    product_id uuid,
    inventory_id uuid,
    FOREIGN KEY (inventory_id) references inventories(id),
    FOREIGN KEY (product_id) references products(id),
    UNIQUE(inventory_id, product_id)
);

CREATE TABLE pricebooks (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    pricebook_name VARCHAR NOT NULL UNIQUE,
    pricebook_reference VARCHAR NOT NULL UNIQUE,
    pricebook_currency_code VARCHAR NOT NULL
);

CREATE TABLE pricebooks_products (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,

    price DECIMAL NOT NULL,

    product_id uuid,
    pricebook_id uuid,
    FOREIGN KEY (pricebook_id) references pricebooks(id),
    FOREIGN KEY (product_id) references products(id),
    UNIQUE(pricebook_id, product_id)
);

CREATE TYPE metadataobjecttype AS ENUM (
  'PRODUCT',
  'CATEGORY',
  'INVENTORY',
  'PRICEBOOK'
);
CREATE TYPE metadatafieldtype AS ENUM (
  'STRING',
  'INT'
);
CREATE TABLE _metadata_custom_fields (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,

  object metadataobjecttype NOT NULL,
  "type" metadatafieldtype NOT NULL,
  name VARCHAR NOT NULL UNIQUE,
  description VARCHAR,
  mandatory boolean NOT NULL DEFAULT false,

  -- string type
  max_len bigint,
  min_len bigint
);

CREATE TABLE _metadata_webhooks (
  id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
  url VARCHAR NOT NULL,
);

CREATE OR REPLACE FUNCTION log_updates() RETURNS TRIGGER AS $$
DECLARE
  json JSONB := '{}';
  changes JSONB := '{}';
  col TEXT;
  old_v TEXT;
  new_v TEXT;
BEGIN
  json := jsonb_set(json, ARRAY['action'], to_jsonb(TG_OP));
  json := jsonb_set(json, ARRAY['entity'], to_jsonb(TG_TABLE_NAME));

  -- ON INSERT AND DELETE
  IF TG_OP IS DISTINCT FROM 'UPDATE' THEN
    IF TG_OP IS NOT DISTINCT FROM 'DELETE' THEN
      json := jsonb_set(json, ARRAY['id'], to_jsonb(OLD.id));
    END IF;

    IF TG_OP IS NOT DISTINCT FROM 'INSERT' THEN
      json := jsonb_set(json, ARRAY['id'], to_jsonb(NEW.id));
    END IF;
  END IF;

  -- ON UPDATE
  IF TG_OP IS NOT DISTINCT FROM 'UPDATE' THEN
    FOR col IN
      SELECT column_name
      FROM information_schema.columns
      WHERE table_name = TG_TABLE_NAME AND column_name NOT IN ('id')
    LOOP
      EXECUTE format('SELECT ($1).%I', col) INTO old_v USING OLD;
      EXECUTE format('SELECT ($1).%I', col) INTO new_v USING NEW;

      IF old_v IS DISTINCT FROM new_v THEN
        changes := jsonb_set(changes, ARRAY[col], to_jsonb(new_v)); 
      END IF;
    END LOOP;

    json := jsonb_set(json, ARRAY['changes'], to_jsonb(changes));
    json := jsonb_set(json, ARRAY['id'], to_jsonb(OLD.id));
  END IF;

  PERFORM pg_notify('table_changes', json::text);

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER notify_update AFTER INSERT OR UPDATE OR DELETE on products
FOR EACH ROW
EXECUTE FUNCTION log_updates();
