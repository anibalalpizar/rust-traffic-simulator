mod metrics;
mod models;
mod simulation;

use crate::models::{Intersection, Road, TrafficSignal};
use crate::simulation::RoadNetwork;
use std::time::Duration;

fn main() {
    let mut network: RoadNetwork = RoadNetwork::new();

    let road1: Road = Road::new(
        1,
        1,
        2,
        1000.0,
        vec![TrafficSignal::SpeedLimit(60.0), TrafficSignal::Stop],
    );

    network.add_road(road1);

    let intersection = Intersection::new(1, (500.0, 500.0), vec![1]);
    network.add_intersection(intersection);

    network.generate_vehicles(50);

    let simulation_time = Duration::from_secs(60);
    let time_step = Duration::from_millis(100);

    let mut elapsed_time = Duration::from_secs(0);
    while elapsed_time < simulation_time {
        network.simulate_traffic(time_step);
        elapsed_time += time_step;
    }

    let metrics = network.calculate_metrics();
    println!("Traffic Metrics:");
    println!("Total Vehicles: {}", metrics.total_vehicles);
    println!("Average Speed: {:.2} km/h", metrics.average_speed);
}
