#[derive(Debug)]
pub enum Status {
    Charging,
    Rolling,
    Parked,
}

#[derive(Debug)]
pub struct Vehicle {
    position: (u32, u32),
    status: Status,
    autonomy: f32,
    battery: f32,
}

impl Vehicle {
    pub fn new(position: (u32, u32), autonomy: f32, battery: f32) -> Self {
        Self {
            position,
            status: Status::Parked,
            autonomy,
            battery,
        }
    }
}
