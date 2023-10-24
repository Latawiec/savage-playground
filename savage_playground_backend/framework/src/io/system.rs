use bevy::prelude::{EventReader, ResMut};

use super::{event::PlayerInputEvent, resource::PlayerInputManager};

pub fn input_system(
    mut ev_player_input: EventReader<PlayerInputEvent>,
    mut res_player_input: ResMut<PlayerInputManager>,
) {
    for ev in ev_player_input.iter() {
        res_player_input.set_input_state(ev.player_id, ev.new_state);
    }
}





#[cfg(test)]
mod test {
    use super::input_system;
    use bevy::ecs::system::assert_is_system;

    #[test]
    fn check_systems_validity() {
        assert_is_system(input_system);
    }
}