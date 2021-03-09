use super::ai_hierarchy::data_components::*;
use super::data_components::*;
use crate::common::map::bundles::PickableBundle;
use bevy::prelude::*;
#[derive(Bundle)]
pub struct BuildingBundle {
    pub name: Name,
    pub in_settlement: InSettlement,
    pub owner: BuildingOwner,
    #[bundle]
    pub pickable: PickableBundle,
    #[bundle]
    pub sprite: SpriteBundle,
}
