use crate::models::traffic_light::TrafficLight;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Intersection {
    pub id: u64,
    pub position: (f32, f32),
    pub traffic_light: Option<TrafficLight>,
    pub connected_roads: Vec<u64>,
}

impl Intersection {
    pub fn new(id: u64, position: (f32, f32), connected_roads: Vec<u64>) -> Self {
        Intersection {
            id,
            position,
            traffic_light: Some(TrafficLight::new()),
            connected_roads,
        }
    }
}
