use serde::{Serialize, Deserialize};
use tokio_postgres::{Row, Error};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProductImage {
    pub id: uuid::Uuid,
    pub src: String,
    pub srcset: Option<String>,
    pub alt: Option<String>,
    pub product_id: uuid::Uuid,
}

impl From<&Row> for ProductImage {
    fn from(value: &Row) -> Self {
        return Self {
            id: value.get("image_id"),
            src: value.get("src"),
            srcset: value.try_get("srcset").map_or(None, |x| Some(x)),
            alt: value.try_get("alt").map_or(None, |x| Some(x)),
            product_id: value.get("product_id")
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: uuid::Uuid,
    pub product_name: String,
    pub product_description: String,
    pub product_color: Option<String>,
    pub product_images: Option<Vec<ProductImage>>,
    pub product_custom_fields: Option<Vec<String>>
}

impl From<&Row> for Product {
    fn from(value: &Row) -> Self {
        let mut product = Self{
            id: value.get("id"),
            product_name: value.get("product_name"),
            product_description: value.get("product_description"),
            product_color: value
                .try_get("product_color")
                .map_or(None, |x| Some(x)),
            product_images: None,
            product_custom_fields: None
        };

        // this check is needed because everywhere, besides the /product/{product_id} route, we
        // fetch only the first image of the product - thus it being part of the product query response
        let should_parse_image: Result<uuid::Uuid, Error> = value.try_get("image_id");
        if let Ok(_) = should_parse_image {
            let image = ProductImage::try_from(value);
            if let Ok(image) = image {
                product.product_images = Some(vec![image]);
            }
        }

        return product;
    }
}
