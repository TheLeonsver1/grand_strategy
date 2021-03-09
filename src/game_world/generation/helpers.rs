use crate::{
    common::map::{bundles::PickableBundle, data_components::RectFromTransform},
    game_world::{
        ai_hierarchy::data_components::BuildingOwner,
        buildings::{bundles::BuildingBundle, data_components::InSettlement},
    },
    SpriteHandleHashMapResource, TextureSpawnInfo,
};
use bevy::{prelude::*, utils::HashMap};
use rand::Rng;
pub fn spawn_settlement(
    mut commands: Commands,
    game_sprites: Res<SpriteHandleHashMapResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    unique_buildings: Vec<String>,
    center: Vec2,
    ruler: Entity,
    settlement: Entity,
) {
    if let Some(texture_spawn_infos) =
        check_if_textures_exist(unique_buildings.to_owned(), game_sprites.map.to_owned())
    {
        //let center_v3 = center.extend(0.);
        let village_distance_from_manor: Vec2 = Vec2::new(350.0, 200.0);
        let num_of_houses = unique_buildings.len() * 5;
        let mut rng = rand::thread_rng();
        let house_tex_spawn_info = game_sprites.map.get("House").unwrap();
        for i in 0..num_of_houses {
            commands.spawn(BuildingBundle {
                name: Name::new(format!("House {}", i)),
                in_settlement: InSettlement(settlement),
                owner: BuildingOwner(ruler),
                pickable: PickableBundle {
                    rect_from_transform: house_tex_spawn_info.rect.to_owned(),
                    ..Default::default()
                },
                sprite: SpriteBundle {
                    material: materials.add(house_tex_spawn_info.handle.to_owned().into()),
                    transform: Transform {
                        scale: Vec3::new(0.8, 0.8, 1.),
                        translation: Vec3::new(
                            center.x
                                + village_distance_from_manor.x
                                + rng.gen_range(0.0..4.0)
                                + ((i as u32 % 2)
                                    * (house_tex_spawn_info.rect.width / 2 + rng.gen_range(32..37)))
                                    as f32,
                            center.y
                                + village_distance_from_manor.y
                                + rng.gen_range(0.0..4.0)
                                + ((i as u32 / 2)
                                    * (house_tex_spawn_info.rect.height / 2
                                        + rng.gen_range(32..37)))
                                    as f32,
                            0.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            });
        }
        let castle_tex_spawn_info = game_sprites.map.get("Wide Castle").unwrap();

        commands.spawn(BuildingBundle {
            name: Name::new("Random"),
            in_settlement: InSettlement(settlement),
            owner: BuildingOwner(ruler),
            pickable: PickableBundle {
                rect_from_transform: castle_tex_spawn_info.rect.to_owned(),
                ..Default::default()
            },
            sprite: SpriteBundle {
                material: materials.add(castle_tex_spawn_info.handle.to_owned().into()),
                transform: Transform {
                    scale: Vec3::new(0.8, 0.8, 1.),
                    translation: center.extend(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
        });

        //let distance_from_castle= castle_tex_spawn_info.rect.width/2;
        let degree_needed = 360.0 / unique_buildings.len() as f32;
        let mut dir: Vec2 = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        if dir == Vec2::zero() {
            dir = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        }
        let mut normalized_dir = dir.normalize();
        //let point = normalized_dir * Vec2::splat(distance_from_castle as f32);
        for i in 0..texture_spawn_infos.len() {
            let cur_dir = Quat::from_rotation_z(degree_needed.to_radians() * i as f32)
                * normalized_dir.extend(0.);

            let cur_position = center.extend(0.)
                + cur_dir
                    * Vec3::new(
                        (castle_tex_spawn_info.rect.width / 2
                            + texture_spawn_infos[i].rect.width / 2) as f32,
                        (castle_tex_spawn_info.rect.height / 2
                            + texture_spawn_infos[i].rect.height / 2)
                            as f32,
                        0.,
                    );

            commands.spawn(BuildingBundle {
                name: Name::new("Random"),
                in_settlement: InSettlement(settlement),
                owner: BuildingOwner(ruler),
                pickable: PickableBundle {
                    rect_from_transform: texture_spawn_infos[i].rect.to_owned(),
                    ..Default::default()
                },
                sprite: SpriteBundle {
                    material: materials.add(texture_spawn_infos[i].handle.to_owned().into()),
                    transform: Transform {
                        scale: Vec3::new(0.8, 0.8, 1.),
                        translation: cur_position,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            });
        }
    }
}

fn check_if_textures_exist(
    texture_names: Vec<String>,
    game_sprites_map: HashMap<String, TextureSpawnInfo>,
) -> Option<Vec<TextureSpawnInfo>> {
    let mut texture_infos = Vec::<TextureSpawnInfo>::new();
    for tex_name in texture_names {
        match game_sprites_map.get(&tex_name) {
            None => {
                return None;
            }
            Some(texture_spawn_info) => {
                texture_infos.push(texture_spawn_info.to_owned());
            }
        }
    }
    return Some(texture_infos);
}

enum Succeded {
    Yes,
    No,
}
