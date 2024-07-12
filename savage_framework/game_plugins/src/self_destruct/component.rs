use std::time::Duration;

use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct SelfDestruct {
    timer: Timer,
}

impl SelfDestruct {
    pub fn new(time_to_live: Duration) -> SelfDestruct {
        SelfDestruct {
            timer: Timer::new(time_to_live, bevy::time::TimerMode::Once),
        }
    }

    pub fn tick(&mut self, elapsed: Duration) {
        self.timer.tick(elapsed);
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }
}
