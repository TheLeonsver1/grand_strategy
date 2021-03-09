use super::input::resources::MouseWorldPosition;
use super::{
    data_components::RectFromTransform, events::PressedOnEntity, marker_components::Pickable,
};
use bevy::prelude::*;
pub fn clicked_pickable_entity_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut pressed_on_ent_events: ResMut<Events<PressedOnEntity>>,
    mouse_world_position: Res<MouseWorldPosition>,
    q_pickable: Query<(Entity, &Transform, &RectFromTransform), With<Pickable>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left)
        || mouse_button_input.just_pressed(MouseButton::Right)
    {
        for (entity, transform, rect) in q_pickable.iter() {
            if mouse_world_position.position.x < transform.translation.x + rect.width as f32 / 2.
                && mouse_world_position.position.y
                    < transform.translation.y + rect.height as f32 / 2.
                && mouse_world_position.position.x
                    > transform.translation.x - rect.width as f32 / 2.
                && mouse_world_position.position.y
                    > transform.translation.y - rect.height as f32 / 2.
            {
                if mouse_button_input.just_pressed(MouseButton::Right) {
                    pressed_on_ent_events.send(PressedOnEntity {
                        entity,
                        mouse_button: MouseButton::Right,
                    })
                } else if mouse_button_input.just_pressed(MouseButton::Left) {
                    pressed_on_ent_events.send(PressedOnEntity {
                        entity,
                        mouse_button: MouseButton::Left,
                    })
                }
            }
        }
    }
}
