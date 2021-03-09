use crate::common::core::data_components::TargetToMoveTo;
use bevy::prelude::*;
use rand::random;
pub fn assign_target_to_bored_actors(
    mut query: Query<(&mut Transform, &mut TargetToMoveTo)>,
    time: Res<Time>,
) {
    for (mut transform, mut target) in query.iter_mut() {
        if transform.translation.distance(target.target_position) < 3. {
            target.target_position = Vec3::new(
                random::<f32>() * 100. - 50.,
                random::<f32>() * 100. - 50.,
                0.,
            );
        } else {
            let mut direction = target.target_position - transform.translation;
            if Vec3::length(direction) > 0. {
                direction = Vec3::normalize(direction);
                transform.translation += direction * time.delta_seconds() * 15.;
            }
        }
    }
}
