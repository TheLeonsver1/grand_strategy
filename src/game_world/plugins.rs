use super::{
    ai::{
        monsters::marker_components, sapients::marker_components::*, shared::marker_components::*,
    },
    buildings::marker_components::*,
    settlements::marker_components::*,
};
use bevy::{
    ecs::component::{ComponentDescriptor, StorageType},
    prelude::*,
};
pub struct RegisterSparseComponentsPlugin;
impl Plugin for RegisterSparseComponentsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.world_mut()
            .register_component(ComponentDescriptor::new::<IsInGroup>(
                StorageType::SparseSet,
            ))
            .unwrap();
        app.world_mut()
            .register_component(ComponentDescriptor::new::<IsBusy>(StorageType::SparseSet))
            .unwrap();
        app.world_mut()
            .register_component(ComponentDescriptor::new::<IsInsideLocation>(
                StorageType::SparseSet,
            ))
            .unwrap();
        app.world_mut()
            .register_component(ComponentDescriptor::new::<ShouldGoToTargetPosition>(
                StorageType::SparseSet,
            ))
            .unwrap();
    }
}
