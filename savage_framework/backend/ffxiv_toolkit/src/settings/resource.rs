use bevy::prelude::Resource;



#[derive(Resource, Default)]
pub struct GameSettings {
    pub game_master_id: Option<u64>,
}