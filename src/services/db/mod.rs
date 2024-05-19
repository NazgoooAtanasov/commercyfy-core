use crate::models::category::Category;
use crate::models::product::Product;

pub trait DbService {
    async fn get_categories(&self) -> Result<Vec<Category>, sqlx::Error>;
    async fn create_category(
        &self,
        category: crate::schemas::category::CreateCategory,
    ) -> Result<Category, sqlx::Error>;
    async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, sqlx::Error>;
    async fn get_category_by_reference(&self, reference: &str) -> Result<Option<Category>, sqlx::Error>;
    async fn get_category_products_by_id(&self, id: &str) -> Result<Vec<Product>, sqlx::Error>;
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

    async fn get_category_by_reference(&self, reference: &str) -> Result<Option<Category>, sqlx::Error> {
        return sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE category_reference = $1")
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
}
