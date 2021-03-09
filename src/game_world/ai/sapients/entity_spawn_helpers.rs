use crate::common::map::{data_components::RectFromTransform, marker_components::Pickable};
use bevy::{prelude::*, utils::HashMap};

use super::{bundles::*, data_components::*, marker_components::*, prelude::actor_types::Child};

pub fn spawn_dynasty_random_history(commands: &mut Commands) -> Entity {
    commands.spawn(DynastyBundle {
        origin_story: History("Came From Far Away".to_owned()),
        renown: Renown::Peasant,
    });
    return commands.current_entity().unwrap();
}

pub fn spawn_dynasty_with_history(mut commands: &mut Commands, history: String) -> Entity {
    commands.spawn(DynastyBundle {
        origin_story: History(history.to_owned()),
        renown: Renown::Peasant,
    });
    return commands.current_entity().unwrap();
}

pub fn spawn_sapient_character(
    commands: &mut Commands,
    path_to_image: String,
    name: String,
    age: u32,
    sex: Sex,
    transform: Transform,
    dynasty: Entity,
) -> Entity {
    commands.spawn(SapientActorBundle {
        name: CharacterName(name.to_owned()),
        age: Age(age),
        sex,
        of_dynasty: OfDynasty(dynasty),
        _sapient: actor_types::SapientMarker::default(),
        base_actor: BaseActorBundle::default(),
    });
    let entity = commands.current_entity().unwrap();
    if age < 16 {
        commands.insert(entity.to_owned(), Child);
    }
    return entity;
}

pub fn spawn_sapient_with_new_dynasty(
    commands: &mut Commands,
    path_to_image: String,
    name: String,
    age: u32,
    sex: Sex,
    transform: Transform,

    material: Handle<ColorMaterial>,
) -> (Entity, Entity) {
    let dynasty = spawn_dynasty_random_history(commands);
    let sapient = spawn_sapient_character(
        commands,
        path_to_image,
        name,
        age,
        sex,
        transform,
        dynasty.to_owned(),
    );
    return (dynasty, sapient);
}
pub fn spawn_monster() {}
