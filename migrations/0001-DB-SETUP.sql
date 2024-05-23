CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE products (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY, 
    product_name VARCHAR NOT NULL,
    product_description VARCHAR NOT NULL,
    product_color VARCHAR
);

CREATE TABLE product_custom_fields (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY, 

    product_id uuid NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id),

    __meta_custom_field_id uuid NOT NULL,
    FOREIGN KEY (__meta_custom_field_id) REFERENCES __meta_product_custom_fields(id),

    name VARCHAR NOT NULL UNIQUE,
    value VARCHAR
);

CREATE TABLE __meta_product_custom_fields (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY, 
    name VARCHAR NOT NULL UNIQUE, 
    description VARCHAR, 
    value_type VARCHAR NOT NULL, 
    default_value VARCHAR, 
    mandatory BOOLEAN
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
