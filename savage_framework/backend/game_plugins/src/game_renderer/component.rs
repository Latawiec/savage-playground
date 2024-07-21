use std::sync::atomic::{AtomicU64, Ordering};

use bevy::prelude::{Bundle, Component};
use game_renderer::proto::game_renderer::DrawBundle;

static SAVAGE_SCENE_ELEMENT_ID: AtomicU64 = AtomicU64::new(0);
#[derive(Component)]
pub struct SceneElementId {
    id: u64,
}

impl SceneElementId {
    pub fn new() -> SceneElementId {
        SceneElementId {
            id: SAVAGE_SCENE_ELEMENT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Component)]
pub struct RenderComponent {
    current: DrawBundle,
    _previous: DrawBundle, // Use this to only send data that has changed.
}

impl RenderComponent {
    pub fn full_bundle(&self) -> DrawBundle {
        self.current.clone()
    }
}

#[derive(Bundle)]
pub struct SceneElement {
    pub render_component: RenderComponent,
    pub render_identifier: SceneElementId,
}
