use crate::metrics::TrafficMetrics;
use crate::models::{Intersection, Road, TrafficLight, TrafficSignal, Vehicle, VehicleType};
use rand::Rng;
use std::collections::HashMap;
use std::time::Duration;

pub struct RoadNetwork {
    pub roads: HashMap<u64, Road>,
    pub intersections: HashMap<u64, Intersection>,
    pub vehicles: Vec<Vehicle>,
    rng: rand::rngs::ThreadRng,
}

impl RoadNetwork {
    pub fn new() -> Self {
        RoadNetwork {
            roads: HashMap::new(),
            intersections: HashMap::new(),
            vehicles: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn add_road(&mut self, road: Road) {
        self.roads.insert(road.id, road);
    }

    pub fn add_intersection(&mut self, mut intersection: Intersection) {
        if intersection.traffic_light.is_none() {
            intersection.traffic_light = Some(TrafficLight::new());
        }
        self.intersections.insert(intersection.id, intersection);
    }

    pub fn generate_vehicles(&mut self, num_vehicles: usize) {
        for i in 0..num_vehicles {
            let vehicle_type = match self.rng.gen_range(0..4) {
                0 => VehicleType::Car,
                1 => VehicleType::Truck,
                2 => VehicleType::Motorcycle,
                _ => VehicleType::Bus,
            };

            let vehicle = Vehicle {
                id: i as u64,
                vehicle_type,
                position: (
                    self.rng.gen_range(0.0..1000.0),
                    self.rng.gen_range(0.0..1000.0),
                ),
                speed: self.rng.gen_range(10.0..80.0),
                direction: self.rng.gen_range(0.0..360.0),
                current_road: self.rng.gen_range(0..self.roads.len() as u64),
            };

            self.vehicles.push(vehicle);
        }
    }

    pub fn simulate_traffic(&mut self, delta_time: Duration) {
        let delta_seconds = delta_time.as_secs_f32();

        for (_, intersection) in self.intersections.iter_mut() {
            if let Some(traffic_light) = &mut intersection.traffic_light {
                traffic_light.update();
            }
        }

        for vehicle in &mut self.vehicles {
            if let Some(road) = self.roads.get(&vehicle.current_road) {
                let speed_limit = road
                    .traffic_signals
                    .iter()
                    .find_map(|signal| match signal {
                        TrafficSignal::SpeedLimit(limit) => Some(*limit),
                        _ => None,
                    })
                    .unwrap_or(80.0);

                vehicle.speed = vehicle.speed.min(speed_limit);

                vehicle.update_position(delta_seconds);

                vehicle.position.0 = vehicle.position.0 % 1000.0;
                vehicle.position.1 = vehicle.position.1 % 1000.0;
            }
        }
    }

    pub fn calculate_metrics(&mut self) -> TrafficMetrics {
        let mut metrics = TrafficMetrics::new(
            self.vehicles.len(),
            self.vehicles.iter().map(|v| v.speed).sum::<f32>() / self.vehicles.len() as f32,
        );

        let congestion = self
            .roads
            .iter()
            .map(|(road_id, _)| {
                let vehicles_on_road = self
                    .vehicles
                    .iter()
                    .filter(|v| v.current_road == *road_id)
                    .count();

                metrics.add_congestion_point(*road_id, vehicles_on_road as f32);

                vehicles_on_road
            })
            .max()
            .unwrap_or(0);

        if congestion > 10 {
            metrics.add_accident_location(
                self.rng.gen_range(0..self.roads.len() as u64),
                self.rng.gen_range(0.0..1000.0),
            );
        }

        metrics
    }
}

impl Default for RoadNetwork {
    fn default() -> Self {
        Self::new()
    }
}
