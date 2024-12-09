use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
pub struct TrafficLight {
    pub state: TrafficLightState,
    pub cycle_duration: Duration,
    pub last_change: Instant,
}

impl TrafficLight {
    pub fn new() -> Self {
        TrafficLight {
            state: TrafficLightState::Red,
            cycle_duration: Duration::from_secs(30),
            last_change: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.last_change.elapsed();

        if elapsed >= self.cycle_duration {
            self.state = match self.state {
                TrafficLightState::Red => TrafficLightState::Green,
                TrafficLightState::Green => TrafficLightState::Yellow,
                TrafficLightState::Yellow => TrafficLightState::Red,
            };

            self.last_change = Instant::now();
        }
    }
}

impl Default for TrafficLight {
    fn default() -> Self {
        Self::new()
    }
}
