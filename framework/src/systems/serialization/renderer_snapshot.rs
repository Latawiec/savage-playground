use bevy::prelude::{Entity, GlobalTransform, Query, ResMut, Resource};
use serde_json::json;

use crate::components::rendering::drawable::{Sprite, Drawable};

#[derive(Resource, Default)]
pub struct RendererSnapshot {
    snapshot: serde_json::Value,
}

impl RendererSnapshot {
    pub fn update_snapshot(
        mut res: ResMut<RendererSnapshot>,
        drawable: Query<(Entity, &GlobalTransform, &Drawable, Option<&Sprite>)>,
    ) {
        let mut snapshot_map = serde_json::Map::new();

        for (entity, transform, drawable, sprite_opt) in drawable.iter() {
            use serde_json::Value;

            let uuid_string: String = entity.index().to_string();

            // Separate these, so if only translation changed, we dont send either scale or rotation again.
            let (scale, rotation, translation) = transform.to_scale_rotation_translation();
            let scale_value = Value::from(scale.to_array().as_slice());
            let rotation_value = Value::from(rotation.to_array().as_slice());
            let translation_value = Value::from(translation.to_array().as_slice());

            let mut map = serde_json::Map::new();
            map.insert("transform".to_owned(), json!({
                "scale": scale_value,
                "rotation": rotation_value,
                "translation": translation_value,
            }));
            map.insert("drawable".to_owned(), serde_json::to_value(drawable).unwrap());

            if let Some(sprite) = sprite_opt {
                map.insert("sprite".to_owned(), serde_json::to_value(sprite).unwrap());
            }

            snapshot_map.insert(uuid_string, serde_json::Value::Object(map));
        }

        res.snapshot = serde_json::Value::Object(snapshot_map);
    }
}
