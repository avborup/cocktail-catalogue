use once_cell::sync::Lazy;
use serde_json::{json, Value};

#[allow(clippy::declare_interior_mutable_const)]
pub const DAIQUIRI: Lazy<Value> = Lazy::new(|| {
    json!({
        "name": "Daiquiri",
        "ingredients": [
            {
                "label": "White rum",
                "amount": 2.0,
                "unit": "oz"
            },
            {
                "label": "Lime juice",
                "amount": 0.75,
                "unit": "oz"
            },
            {
                "label": "Simple syrup",
                "amount": 0.75,
                "unit": "oz"
            }
        ]
    })
});
