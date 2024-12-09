#[derive(Debug)]
pub struct TrafficMetrics {
    pub total_vehicles: usize,
    pub average_speed: f32,
    pub congestion_points: Vec<(u64, f32)>,
    pub accident_locations: Vec<(u64, f32)>,
}

impl TrafficMetrics {
    pub fn new(total_vehicles: usize, average_speed: f32) -> Self {
        TrafficMetrics {
            total_vehicles,
            average_speed,
            congestion_points: Vec::new(),
            accident_locations: Vec::new(),
        }
    }

    pub fn add_congestion_point(&mut self, road_id: u64, congestion_level: f32) {
        if congestion_level > 0.0 {
            self.congestion_points.push((road_id, congestion_level));
        }
    }

    pub fn add_accident_location(&mut self, road_id: u64, position: f32) {
        self.accident_locations.push((road_id, position));
    }
}
