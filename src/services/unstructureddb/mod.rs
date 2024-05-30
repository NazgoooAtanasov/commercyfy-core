pub mod entry;

use self::entry::UnstructuredEntry;
use crate::models::base_extensions::FieldExtensionObject;
use futures::TryStreamExt;
use mongodb::bson::doc;

pub type UnstructuredDbResult = Result<(), String>;
pub type UnstructuredDbObjectResult = Result<Vec<UnstructuredEntry>, String>;

pub trait UnstructuredDb {
    async fn put_custom_fields(
        &self,
        object: FieldExtensionObject,
        entry: Vec<UnstructuredEntry>,
    ) -> UnstructuredDbResult;

    async fn get_custom_fields(
        &self,
        object: FieldExtensionObject,
        extr_ref: &str,
    ) -> UnstructuredDbObjectResult;
}

pub struct MongoDb {
    db: mongodb::Database,
    required_collections: Vec<String>,
}

impl MongoDb {
    pub fn new(db: mongodb::Database) -> Self {
        return Self {
            db,
            required_collections: vec![
                "products".to_string(),
                "categories".to_string(),
                "inventories".to_string(),
                "pricebooks".to_string(),
            ],
        };
    }

    pub async fn validate_collections(&self) -> Result<(), String> {
        let collection_names = match self.db.list_collection_names(doc! {}).await {
            Ok(names) => names,
            Err(err) => return Err(err.to_string()),
        };

        let non_existing_collections = self
            .required_collections
            .iter()
            .filter(|x| !collection_names.contains(x))
            .map(|x| (x, self.db.create_collection(x, None)));

        for (name, creation) in non_existing_collections {
            if let Err(err) = creation.await {
                return Err(err.to_string());
            }

            println!("Created unstructured db collection '{name}'");
        }

        return Ok(());
    }

    fn get_collection(
        &self,
        object: FieldExtensionObject,
    ) -> mongodb::Collection<UnstructuredEntry> {
        return match object {
            FieldExtensionObject::PRODUCT => self.db.collection("products"),
            FieldExtensionObject::CATEGORY => self.db.collection("categories"),
            FieldExtensionObject::INVENTORY => self.db.collection("inventories"),
            FieldExtensionObject::PRICEBOOK => self.db.collection("pricebooks"),
        };
    }
}
impl UnstructuredDb for MongoDb {
    async fn put_custom_fields(
        &self,
        object: FieldExtensionObject,
        entry: Vec<UnstructuredEntry>,
    ) -> UnstructuredDbResult {
        let collection = self.get_collection(object);
        let insertion = collection.insert_many(entry, None).await;

        if let Err(err) = insertion {
            return Err(err.to_string());
        }

        return Ok(());
    }

    async fn get_custom_fields(
        &self,
        object: FieldExtensionObject,
        extr_ref: &str,
    ) -> UnstructuredDbObjectResult {
        let collection = self.get_collection(object);

        let collection_cursor = match collection.find(doc! { "extr_ref": extr_ref }, None).await {
            Ok(cursor) => cursor,
            Err(err) => return Err(err.to_string()),
        };

        match collection_cursor
            .try_collect::<Vec<UnstructuredEntry>>()
            .await
        {
            Ok(fields) => return Ok(fields),
            Err(err) => return Err(err.to_string()),
        };
    }
}
