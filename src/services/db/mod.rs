use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use sqlx::QueryBuilder;

use crate::models::{category::Category, portal_user::PortalUsersRoles};
use crate::models::inventory::{Inventory, ProductInventoryRecord};
use crate::models::portal_user::PortalUser;
use crate::models::pricebook::{Pricebook, PricebookRecord};
use crate::models::product::{Product, ProductImage};
use crate::schemas::inventory::{CreateInventory, CreateInventoryRecord};
use crate::schemas::portal_user::PortalUserCreate;
use crate::schemas::pricebook::{CreatePricebook, CreatePricebookRecord};
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

    async fn get_inventories(&self) -> DbServiceResult<Vec<Inventory>>;

    async fn get_inventory_by_id(&self, id: &str) -> DbServiceResult<Option<Inventory>>;

    async fn get_inventory_by_reference(
        &self,
        refernece: &str,
    ) -> DbServiceResult<Option<Inventory>>;

    async fn get_inventory_records(&self, id: &str)
        -> DbServiceResult<Vec<ProductInventoryRecord>>;

    async fn create_inventory(&self, payload: CreateInventory) -> DbServiceResult<Inventory>;

    async fn get_product_inventory_record(
        &self,
        product_id: &str,
        inventory_id: &str,
    ) -> DbServiceResult<Option<ProductInventoryRecord>>;

    async fn create_product_inventory_record(
        &self,
        payload: CreateInventoryRecord,
    ) -> DbServiceResult<ProductInventoryRecord>;

    async fn get_pricebooks(&self) -> DbServiceResult<Vec<Pricebook>>;

    async fn get_pricebook_by_id(&self, id: &str) -> DbServiceResult<Option<Pricebook>>;

    async fn get_pricebook_by_reference(
        &self,
        reference: &str,
    ) -> DbServiceResult<Option<Pricebook>>;

    async fn create_pricebook(&self, payload: CreatePricebook) -> DbServiceResult<Pricebook>;

    async fn create_product_pricebook_record(
        &self,
        payload: CreatePricebookRecord,
    ) -> DbServiceResult<PricebookRecord>;

    async fn get_product_pricebook_record(
        &self,
        product_id: &str,
        pricebook_id: &str,
    ) -> DbServiceResult<Option<PricebookRecord>>;

    async fn get_portal_user(&self, id: &str) -> DbServiceResult<Option<PortalUser>>;

    async fn create_portal_user(&self, payload: PortalUserCreate) -> DbServiceResult<PortalUser>;

    async fn get_portal_user_by_email(&self, email: &str) -> DbServiceResult<Option<PortalUser>>;
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

    async fn get_inventory_by_id(&self, id: &str) -> DbServiceResult<Option<Inventory>> {
        return sqlx::query_as::<_, Inventory>("SELECT * FROM inventories WHERE id::text = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;
    }

    async fn get_inventory_by_reference(
        &self,
        refernece: &str,
    ) -> DbServiceResult<Option<Inventory>> {
        return sqlx::query_as::<_, Inventory>(
            "SELECT * FROM inventories WHERE inventory_reference = $1",
        )
        .bind(refernece)
        .fetch_optional(&self.pool)
        .await;
    }

    async fn get_inventories(&self) -> DbServiceResult<Vec<Inventory>> {
        return sqlx::query_as::<_, Inventory>("SELECT * FROM inventories")
            .fetch_all(&self.pool)
            .await;
    }

    async fn get_inventory_records(
        &self,
        id: &str,
    ) -> DbServiceResult<Vec<ProductInventoryRecord>> {
        return sqlx::query_as::<_, ProductInventoryRecord>("SELECT * FROM inventories_products ip JOIN inventories i on i.id = ip.inventory_id WHERE ip.inventory_id::text = $1")
            .bind(id)
            .fetch_all(&self.pool).await;
    }

    async fn create_inventory(&self, payload: CreateInventory) -> DbServiceResult<Inventory> {
        return sqlx::query_as::<_, Inventory>(
            "INSERT INTO inventories (inventory_name, inventory_reference) values ($1, $2) RETURNING *",
        )
        .bind(payload.inventory_name)
        .bind(payload.inventory_reference)
        .fetch_one(&self.pool)
        .await;
    }

    async fn get_product_inventory_record(
        &self,
        product_id: &str,
        inventory_id: &str,
    ) -> DbServiceResult<Option<ProductInventoryRecord>> {
        return sqlx::query_as::<_, ProductInventoryRecord>("SELECT * FROM inventories_products WHERE product_id::text = $1 AND inventory_id::text = $2")
            .bind(product_id)
            .bind(inventory_id)
            .fetch_optional(&self.pool).await;
    }

    async fn create_product_inventory_record(
        &self,
        payload: CreateInventoryRecord,
    ) -> DbServiceResult<ProductInventoryRecord> {
        return sqlx::query_as::<_, ProductInventoryRecord>("INSERT INTO inventories_products (allocation, product_id, inventory_id) VALUES ($1, $2, $3) RETURNING *")
            .bind(payload.allocation)
            .bind(payload.product_id)
            .bind(payload.inventory_id)
            .fetch_one(&self.pool).await;
    }

    async fn get_pricebooks(&self) -> DbServiceResult<Vec<Pricebook>> {
        return sqlx::query_as::<_, Pricebook>("SELECT * FROM pricebooks")
            .fetch_all(&self.pool)
            .await;
    }

    async fn get_pricebook_by_id(&self, id: &str) -> DbServiceResult<Option<Pricebook>> {
        return sqlx::query_as::<_, Pricebook>("SELECT * FROM pricebooks WHERE id::text = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;
    }

    async fn get_pricebook_by_reference(
        &self,
        refernece: &str,
    ) -> DbServiceResult<Option<Pricebook>> {
        return sqlx::query_as::<_, Pricebook>(
            "SELECT * FROM pricebooks WHERE pricebook_reference = $1",
        )
        .bind(refernece)
        .fetch_optional(&self.pool)
        .await;
    }

    async fn create_pricebook(&self, payload: CreatePricebook) -> DbServiceResult<Pricebook> {
        return sqlx::query_as::<_, Pricebook>("INSERT INTO pricebooks (pricebook_name, pricebook_reference, pricebook_currency_code) VALUES ($1, $2, $3) RETURNING *")
            .bind(payload.pricebook_name)
            .bind(payload.pricebook_reference)
            .bind(payload.pricebook_currency_code)
            .fetch_one(&self.pool).await;
    }

    async fn create_product_pricebook_record(
        &self,
        payload: CreatePricebookRecord,
    ) -> DbServiceResult<PricebookRecord> {
        return sqlx::query_as::<_, PricebookRecord>("INSERT INTO pricebooks_products (product_id, pricebook_id, price) VALUES ($1::uuid, $2::uuid, $3) RETURNING *")
            .bind(payload.product_id)
            .bind(payload.pricebook_id)
            .bind(payload.price)
            .fetch_one(&self.pool).await;
    }

    async fn get_product_pricebook_record(
        &self,
        product_id: &str,
        pricebook_id: &str,
    ) -> DbServiceResult<Option<PricebookRecord>> {
        return sqlx::query_as::<_, PricebookRecord>("SELECT * FROM pricebooks_products WHERE product_id::text = $1 AND pricebook_id::text = $2")
            .bind(product_id)
            .bind(pricebook_id)
            .fetch_optional(&self.pool)
            .await;
    }

    async fn get_portal_user(&self, id: &str) -> DbServiceResult<Option<PortalUser>> {
        return sqlx::query_as::<_, PortalUser>("SELECT * FROM portal_users WHERE id::text = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await;
    }

    async fn create_portal_user(&self, payload: PortalUserCreate) -> DbServiceResult<PortalUser> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt);
        if let Err(_) = password_hash {
            return Err(sqlx::Error::Io(std::io::ErrorKind::InvalidInput.into()));
        }

        let hash = password_hash.unwrap().to_string();

        return sqlx::query_as::<_, PortalUser>("INSERT INTO portal_users (email, first_name, last_name, password, roles) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(payload.email)
            .bind(payload.first_name)
            .bind(payload.last_name)
            .bind(hash)
            .bind(payload.roles)
            .fetch_one(&self.pool).await;
    }

    async fn get_portal_user_by_email(&self, email: &str) -> DbServiceResult<Option<PortalUser>> {
        return sqlx::query_as::<_, PortalUser>("SELECT * FROM portal_users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await;
    }
}
