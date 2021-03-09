use super::data_components::*;
use super::marker_components::BuildingMarker;
use super::{ai_hierarchy::data_components::BuildingOwner, bundles::*};
use crate::{
    common::map::{
        bundles::PickableBundle, data_components::RectFromTransform, marker_components::Pickable,
    },
    SpriteHandleHashMapResource,
};
use bevy::{ecs::component::Component, prelude::*, utils::HashMap};

pub fn spawn_building(
    commands: &mut Commands,
    path_to_image: String,
    name: String,
    owner: Entity,
    in_settlement: Entity,
    transform: Transform,
    texures_res: Res<SpriteHandleHashMapResource>,
    materials: ResMut<Assets<ColorMaterial>>,
    building_marker_component: impl BuildingMarker + Component,
) {
    commands
        .spawn(BuildingBundle {
            name: Name::new(name.to_owned()),
            in_settlement: InSettlement(in_settlement.to_owned()),
            owner: BuildingOwner(owner.to_owned()),
            pickable: PickableBundle::default(),
            sprite: SpriteBundle::default(),
        })
        .with(building_marker_component);
}
/*
///note: building_vec takes:(name,settlement,owner,key_to_svg_map,transform,marker_component)
pub fn spawn_buildings(
    mut commands: Commands,
    shared_material: Handle<ColorMaterial>,
    buildings_vec: Vec<(
        String,
        Entity,
        Entity,
        String,
        Transform,
        impl BuildingMarker + Component,
    )>,
) {
    for (name, settlement, owner, path_to_image, transform, marker_component) in buildings_vec {
        spawn_building(
            &mut commands,
            path_to_image,
            name,
            owner,
            settlement,
            transform,
            shared_material.to_owned(),
            marker_component,
        );
    }
}
 */
