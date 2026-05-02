#[tideorm::model(table = "discovery_products")]
#[unique_index("sku")]
pub struct CatalogProduct {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub sku: String,
    pub name: String,
}

impl CatalogProduct {
    pub fn new(sku: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: 0,
            sku: sku.into(),
            name: name.into(),
        }
    }
}
