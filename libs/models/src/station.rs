use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Station {
    pub position: (f32, f32),
    pub power: f32,   // kW
    pub price: f32,   // price per kWh
    pub name: String, // franchise name
}

impl Station {
    pub fn new(position: (f32, f32), power: f32, price: f32, name: String ) -> Self {
        Self {
            position,
            power,
            price,
            name,
        }
    }
     pub fn equals(&self, other: &Station) -> bool{
        self.name == other.name
        && self.power == other.power
        && self.price == other.price
        && self.position == other.position
    }
}

impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name {}, position, ({}, {}), power {}, price/kWh {}",
            self.name, self.position.0, self.position.1, self.power, self.price,
        )
    }
}

