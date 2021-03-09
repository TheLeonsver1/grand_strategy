use super::data_components::*;
use super::marker_components::*;
use crate::common::{
    core::{data_components::TargetToMoveTo, marker_components::Actor},
    map::bundles::PickableBundle,
};
use bevy::prelude::*;
//use bevy::bevy_ecs_macros::Bundle;

#[derive(Bundle, Default)]
pub struct BaseActorBundle {
    pub _actor: Actor,
    pub target_position: TargetToMoveTo,
    #[bundle]
    pub sprite: SpriteBundle,
    #[bundle]
    pub pickable: PickableBundle,
}
