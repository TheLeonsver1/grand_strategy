use crate::{
    common::{self, map::bundles::PickableBundle},
    game_world::{
        ai::{
            sapients::{
                bundles::{DynastyBundle, SapientActorBundle},
                data_components::{OfDynasty, Renown},
            },
            shared::{
                bundles::BaseActorBundle,
                data_components::{Age, History},
                marker_components::Sex,
            },
        },
        ai_hierarchy::data_components::{BuildingOwner, SettlementRuler},
        buildings::{bundles::BuildingBundle, data_components::InSettlement},
        settlements::bundles::AutocraticSettlementBundle,
    },
    GameState, SpriteHandleHashMapResource, TextureSpawnInfo,
};
use bevy::{prelude::*, utils::HashMap};
pub fn test_village_spawn(
    mut commands: Commands,
    game_sprites: Res<SpriteHandleHashMapResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let texture_names: Vec<String> = vec![
        "Background".to_string(),
        "Circle Element".to_string(),
        "Church".to_string(),
        "Farm".to_string(),
        "House".to_string(),
    ];
    if let Succeded::Yes =
        check_if_textures_exist(texture_names.to_owned(), game_sprites.map.to_owned())
    {
        commands.spawn(SpriteBundle {
            transform: Transform::from_scale(Vec3::new(3., 3., 1.)),
            material: materials.add(
                game_sprites
                    .map
                    .to_owned()
                    .get(&texture_names[0].to_owned())
                    .unwrap()
                    .handle
                    .to_owned()
                    .into(),
            ),
            ..Default::default()
        });
        commands.spawn(DynastyBundle {
            origin_story: History("always barons".to_owned()),
            renown: Renown::Baron,
        });
        let dynasty = commands.current_entity().unwrap();

        let texture_spawn_info = game_sprites.map.get(&texture_names[1].to_owned()).unwrap();
        commands.spawn(SapientActorBundle {
            name: Name::new("Baron".to_owned()),
            age: Age(43),
            sex: Sex::Male,
            of_dynasty: OfDynasty(dynasty),
            _sapient: Default::default(),
            base_actor: BaseActorBundle {
                pickable: PickableBundle {
                    rect_from_transform: texture_spawn_info.rect.to_owned(),
                    ..Default::default()
                },
                sprite: SpriteBundle {
                    material: materials.add(texture_spawn_info.handle.to_owned().into()),
                    transform: Transform::from_scale(Vec3::new(0.25, 0.25, 1.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        });
        let ruler_entity = commands.current_entity().unwrap();
        commands.spawn(AutocraticSettlementBundle {
            name: Name::new("TestVillage".to_owned()),
            ruler: SettlementRuler(ruler_entity),
        });
        let settlement_entity = commands.current_entity().unwrap();
        let texture_spawn_info = game_sprites.map.get(&texture_names[2].to_owned()).unwrap();
        commands.spawn(BuildingBundle {
            name: Name::new("Church"),
            in_settlement: InSettlement(settlement_entity),
            owner: BuildingOwner(ruler_entity),
            pickable: PickableBundle {
                rect_from_transform: texture_spawn_info.rect.to_owned(),
                ..Default::default()
            },
            sprite: SpriteBundle {
                material: materials.add(texture_spawn_info.handle.to_owned().into()),
                transform: Transform::from_scale(Vec3::new(0.8, 0.8, 1.)),
                ..Default::default()
            },
        });

        let texture_spawn_info = game_sprites.map.get(&texture_names[3].to_owned()).unwrap();

        commands.spawn(BuildingBundle {
            name: Name::new("Farm"),
            in_settlement: InSettlement(settlement_entity),
            owner: BuildingOwner(ruler_entity),
            pickable: PickableBundle {
                rect_from_transform: texture_spawn_info.rect.to_owned(),
                ..Default::default()
            },
            sprite: SpriteBundle {
                material: materials.add(texture_spawn_info.handle.to_owned().into()),
                transform: Transform {
                    scale: Vec3::new(0.75, 0.75, 1.),
                    translation: Vec3::new(-100., -150., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
        });

        for i in 0..10 {
            let texture_spawn_info = game_sprites.map.get(&texture_names[1].to_owned()).unwrap();
            commands.spawn(SapientActorBundle {
                name: Name::new("Farmer".to_owned()),
                age: Age(43),
                sex: Sex::Male,
                of_dynasty: OfDynasty(dynasty),
                _sapient: Default::default(),
                base_actor: BaseActorBundle {
                    pickable: PickableBundle {
                        rect_from_transform: texture_spawn_info.rect.to_owned(),
                        ..Default::default()
                    },
                    sprite: SpriteBundle {
                        material: materials.add(texture_spawn_info.handle.to_owned().into()),
                        transform: Transform::from_scale(Vec3::new(0.25, 0.25, 1.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            });

            let father = commands.current_entity().unwrap();

            commands.spawn(SapientActorBundle {
                name: Name::new("Farmer".to_owned()),
                age: Age(43),
                sex: Sex::Female,
                of_dynasty: OfDynasty(dynasty),
                _sapient: Default::default(),
                base_actor: BaseActorBundle {
                    pickable: PickableBundle {
                        rect_from_transform: texture_spawn_info.rect.to_owned(),
                        ..Default::default()
                    },
                    sprite: SpriteBundle {
                        material: materials.add(texture_spawn_info.handle.to_owned().into()),
                        transform: Transform::from_scale(Vec3::new(0.25, 0.25, 1.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            });

            let mother = commands.current_entity().unwrap();

            let texture_spawn_info = game_sprites.map.get(&texture_names[4].to_owned()).unwrap();
        }
        game_state.set_next(GameState::InGame);
    }
}
pub(crate) fn test_village_spawn_2(
    mut commands: Commands,
    game_sprites: Res<SpriteHandleHashMapResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let texture_names: Vec<String> = vec![
        "Background".to_string(),
        "Circle Element".to_string(),
        "Church".to_string(),
        "Farm".to_string(),
        "Mill".to_string(),
        "Blacksmith".to_string(),
        "House".to_string(),
    ];
    if let Succeded::Yes =
        check_if_textures_exist(texture_names.to_owned(), game_sprites.map.to_owned())
    {
        commands.spawn(SpriteBundle {
            transform: Transform::from_scale(Vec3::new(3., 3., 1.)),
            material: materials.add(
                game_sprites
                    .map
                    .to_owned()
                    .get(&texture_names[0].to_owned())
                    .unwrap()
                    .handle
                    .to_owned()
                    .into(),
            ),
            ..Default::default()
        });
        commands.spawn(DynastyBundle {
            origin_story: History("always barons".to_owned()),
            renown: Renown::Baron,
        });
        let dynasty = commands.current_entity().unwrap();

        let texture_spawn_info = game_sprites.map.get(&texture_names[1].to_owned()).unwrap();
        commands.spawn(SapientActorBundle {
            name: Name::new("Baron".to_owned()),
            age: Age(43),
            sex: Sex::Male,
            of_dynasty: OfDynasty(dynasty),
            _sapient: Default::default(),
            base_actor: BaseActorBundle {
                pickable: PickableBundle {
                    rect_from_transform: texture_spawn_info.rect.to_owned(),
                    ..Default::default()
                },
                sprite: SpriteBundle {
                    material: materials.add(texture_spawn_info.handle.to_owned().into()),
                    transform: Transform::from_scale(Vec3::new(0.25, 0.25, 1.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        });
        let ruler_entity = commands.current_entity().unwrap();
        commands.spawn(AutocraticSettlementBundle {
            name: Name::new("TestVillage".to_owned()),
            ruler: SettlementRuler(ruler_entity),
        });
        let settlement_entity = commands.current_entity().unwrap();
        super::helpers::spawn_settlement(
            commands,
            game_sprites,
            materials,
            vec![
                texture_names[2].to_owned(),
                texture_names[3].to_owned(),
                texture_names[4].to_owned(),
                texture_names[5].to_owned(),
            ],
            Vec2::new(0., 0.),
            ruler_entity,
            settlement_entity,
        );
    }
    game_state.set_next(GameState::InGame);
}
fn check_if_textures_exist(
    texture_names: Vec<String>,
    game_sprites_map: HashMap<String, TextureSpawnInfo>,
) -> Succeded {
    for tex_name in texture_names {
        if let None = game_sprites_map.get(&tex_name) {
            return Succeded::No;
        }
    }
    return Succeded::Yes;
}
enum Succeded {
    Yes,
    No,
}
