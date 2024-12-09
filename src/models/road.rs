#[derive(Debug, Clone)]
pub enum TrafficSignal {
    Stop,
    SpeedLimit(f32),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Road {
    pub id: u64,
    pub start_intersection: u64,
    pub end_intersection: u64,
    pub length: f32,
    pub traffic_signals: Vec<TrafficSignal>,
}
impl Road {
    pub fn new(id: u64, start: u64, end: u64, length: f32, signals: Vec<TrafficSignal>) -> Self {
        Road {
            id,
            start_intersection: start,
            end_intersection: end,
            length,
            traffic_signals: signals,
        }
    }
}
