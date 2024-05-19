use sqlx::QueryBuilder;

use crate::models::category::Category;
use crate::models::product::{Product, ProductImage};
use crate::schemas::product::{CreateProduct, CreateProductImage};

type DbServiceResult<T> = Result<T, sqlx::Error>;

pub trait DbService {
    async fn get_categories(&self) -> DbServiceResult<Vec<Category>>;

    async fn create_category(
        &self,
        category: crate::schemas::category::CreateCategory,
    ) -> DbServiceResult<Category>;

    async fn get_category_by_id(&self, id: &str) -> DbServiceResult<Option<Category>>;

    async fn get_category_by_reference(&self, reference: &str)
        -> DbServiceResult<Option<Category>>;

    async fn get_category_products_by_id(&self, id: &str) -> DbServiceResult<Vec<Product>>;

    async fn get_product(&self, id: &str) -> DbServiceResult<Option<Product>>;

    async fn get_product_images(&self, id: &str) -> DbServiceResult<Vec<ProductImage>>;

    async fn create_product(&self, payload: CreateProduct) -> DbServiceResult<Product>;

    async fn create_product_category_assignment(
        &self,
        product_id: uuid::Uuid,
        categories: Vec<uuid::Uuid>,
    ) -> DbServiceResult<()>;

    async fn create_product_image(
        &self,
        id: &str,
        payload: CreateProductImage,
    ) -> DbServiceResult<ProductImage>;
}

pub struct PgDbService {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PgDbService {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        return Self { pool };
    }
}

impl DbService for PgDbService {
    async fn get_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
        return sqlx::query_as::<_, Category>("SELECT * FROM categories")
            .fetch_all(&self.pool)
            .await;
    }

    async fn create_category(
        &self,
        category: crate::schemas::category::CreateCategory,
    ) -> Result<Category, sqlx::Error> {
        return sqlx::query_as("INSERT INTO categories (category_name, category_description, category_reference) VALUES ($1, $2, $3) RETURNING *")
            .bind(category.category_name)
            .bind(category.category_description)
            .bind(category.category_reference)
            .fetch_one(&self.pool).await;
    }

    async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, sqlx::Error> {
        return sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id::text = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;
    }

    async fn get_category_by_reference(
        &self,
        reference: &str,
    ) -> Result<Option<Category>, sqlx::Error> {
        return sqlx::query_as::<_, Category>(
            "SELECT * FROM categories WHERE category_reference = $1",
        )
        .bind(reference)
        .fetch_optional(&self.pool)
        .await;
    }

    async fn get_category_products_by_id(&self, id: &str) -> Result<Vec<Product>, sqlx::Error> {
        return sqlx::query_as::<_, Product>("SELECT p.* FROM products p JOIN categories_products cp on cp.product_id = p.id WHERE cp.category_id::text = $1")
            .bind(id)
            .fetch_all(&self.pool)
            .await;
    }

    async fn get_product(&self, id: &str) -> DbServiceResult<Option<Product>> {
        return sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id::text = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;
    }

    async fn get_product_images(&self, id: &str) -> DbServiceResult<Vec<ProductImage>> {
        return sqlx::query_as::<_, ProductImage>(
            "SELECT id, src, srcset, alt, product_id FROM images WHERE product_id::text = $1",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await;
    }

    async fn create_product(&self, payload: CreateProduct) -> DbServiceResult<Product> {
        return sqlx::query_as::<_, Product>("INSERT INTO products (product_name, product_description, product_color) VALUES ($1, $2, $3) RETURNING *")
            .bind(payload.product_name)
            .bind(payload.product_description)
            .bind(payload.product_color)
            .fetch_one(&self.pool)
            .await;
    }

    async fn create_product_category_assignment(
        &self,
        product_id: uuid::Uuid,
        categories: Vec<uuid::Uuid>,
    ) -> DbServiceResult<()> {
        let mut builder =
            QueryBuilder::new("INSERT INTO categories_products (category_id, product_id)");

        builder.push_values(categories.into_iter(), |mut b, uuid| {
            b.push_bind(uuid).push_bind(product_id);
        });

        let query = builder.build();

        let result = query.execute(&self.pool).await;
        if let Err(error) = result {
            return Err(error);
        }

        return Ok(());
    }

    async fn create_product_image(
        &self,
        id: &str,
        payload: CreateProductImage,
    ) -> DbServiceResult<ProductImage> {
        return sqlx::query_as::<_, ProductImage>(
            "INSERT INTO images (product_id, src, srcset, alt) VALUES ($1::uuid, $2, $3, $4) RETURNING *",
        )
        .bind(id)
        .bind(payload.src)
        .bind(payload.srcset)
        .bind(payload.alt)
        .fetch_one(&self.pool)
        .await;
    }
}
