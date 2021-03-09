use super::{
    data_components::*,
    marker_components::*,
    shared::{bundles::BaseActorBundle, data_components::*, marker_components::*},
};
use crate::common::{
    core::{data_components::TargetToMoveTo, marker_components::Actor},
    map::bundles::PickableBundle,
};
use bevy::prelude::*;
#[derive(Bundle)]
pub struct DynastyBundle {
    pub origin_story: History,
    pub renown: Renown,
}
#[derive(Bundle)]
pub struct SapientActorBundle {
    pub of_dynasty: OfDynasty,
    pub name: Name,
    pub age: Age,
    pub sex: Sex,
    pub _sapient: SapientMarker,
    #[bundle]
    pub base_actor: BaseActorBundle,
}
