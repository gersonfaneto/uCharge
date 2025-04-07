use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Station {
    pub position: (f32, f32),
    pub power: f32,   // kW
    pub price: f32,   // price per kWh
    pub name: String, // franchise name
}

impl Station {
    pub fn new(position: (f32, f32), power: f32, price: f32, name: String) -> Self {
        Self {
            position,
            power,
            price,
            name,
        }
    }
}
