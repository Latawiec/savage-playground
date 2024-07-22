use std::time::Duration;

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TimeSlicedIoConfig {
    pub io_exchange_duration: Duration,
}