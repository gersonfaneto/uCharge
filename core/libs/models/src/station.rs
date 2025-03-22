#[derive(Debug)]
pub enum Status {
    Free,
    Charging,
    Occupied,
}

#[derive(Debug)]
pub struct Station {
    position: (u32, u32),
    status: Status,
    fuel_rate: f32,
}

impl Station {
    pub fn new(position: (u32, u32), fuel_rate: f32) -> Self {
        Self {
            position,
            status: Status::Free,
            fuel_rate,
        }
    }
}
