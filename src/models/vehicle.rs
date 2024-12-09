#[derive(Debug, Clone, PartialEq)]
pub enum VehicleType {
    Car,
    Truck,
    Motorcycle,
    Bus,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Vehicle {
    pub id: u64,
    pub vehicle_type: VehicleType,
    pub position: (f32, f32),
    pub speed: f32,
    pub direction: f32,
    pub current_road: u64,
}

impl Vehicle {
    pub fn update_position(&mut self, delta_time: f32) {
        let dx = self.speed * delta_time * self.direction.to_radians().cos();
        let dy = self.speed * delta_time * self.direction.to_radians().sin();

        self.position.0 += dx;
        self.position.1 += dy;
    }
}
