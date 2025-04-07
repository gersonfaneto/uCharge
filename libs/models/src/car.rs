use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Car {
    pub battery_capacity: f32, // kWh
    pub autonomy: f32,         // km/kWh
    pub charging_power: f32,   // Wh
    pub position: (f32, f32),  // position x,y
    pub battery_level: f32,    // %
}

impl Car {
    pub fn new(
        battery_capacity: f32,
        autonomy: f32,
        charging_power: f32,
        position: (f32, f32),
        battery_level: f32,
    ) -> Self {
        Self {
            battery_capacity,
            autonomy,
            charging_power,
            position,
            battery_level,
        }
    }

    pub fn increment_battery_level(mut self, amount: f32) {
        let new_level = self.battery_level + amount;
        match new_level {
            (..=0.0) => self.battery_level = 0.0,
            (0.0..=1.0) => self.battery_level = new_level,
            _ => self.battery_level = 100.0,
        }
    }

    pub fn set_battery_level(mut self, level: f32) {
        if (0.0..=100.0).contains(&level) {
            self.battery_level = level;
        }
    }

    // moves self in the given direction consuming battery
    pub fn move_in(mut self, direction: (f32, f32)) {
        let vector_module = (direction.0.exp2() + direction.1.exp2()).sqrt();
        let versor = (direction.0 / vector_module, direction.1 / vector_module);

        let max_km = self.battery_capacity * self.battery_level * self.autonomy;

        let distance = if max_km < vector_module {
            max_km
        } else {
            vector_module
        };

        let movement = (versor.0 * distance, versor.1 * distance);

        self.position.0 += movement.0;
        self.position.1 += movement.1;

        self.reduce_battery(distance);
    }

    fn reduce_battery(mut self, distance: f32) {
        self.battery_level = (distance * self.autonomy) / self.battery_capacity;
    }
}
