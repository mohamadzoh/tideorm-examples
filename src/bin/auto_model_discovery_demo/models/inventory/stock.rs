#[tideorm::model(table = "discovery_stock")]
#[index("product_id")]
#[index("warehouse")]
pub struct WarehouseStock {
    #[tideorm(primary_key, auto_increment)]
    pub id: i64,
    pub product_id: i64,
    pub warehouse: String,
    pub on_hand: i32,
}

impl WarehouseStock {
    pub fn new(product_id: i64, warehouse: impl Into<String>, on_hand: i32) -> Self {
        Self {
            id: 0,
            product_id,
            warehouse: warehouse.into(),
            on_hand,
        }
    }
}
