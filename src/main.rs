pub mod common;
pub mod game_ui;
pub mod game_world;

use std::path::{Path, PathBuf};

use bevy::{
    asset::{
        AssetLoader, AssetPath, AssetPathId, BoxedFuture, LoadContext, LoadState, LoadedAsset,
    },
    prelude::*,
    reflect::TypeUuid,
    utils::HashMap,
};
use common::{core::marker_components::MainCamera, map::data_components::RectFromTransform};
use game_world::{ai, ai_hierarchy, buildings, settlements};
use ron;
use serde::Deserialize;
#[derive(Debug, Clone)]
pub enum GameState {
    SetupStart,
    SetupLoadAssets,
    SetupFinish,
    MainMenu,
    GenerateNewWorld,
    InGame,
    Load(String),
    Save(String),
}
static STATE: &str = "state";
fn main() {
    App::build()
        //add msaa to make sprites look better
        //.insert_resource(Msaa { samples: 8 })
        .insert_resource(State::new(GameState::SetupStart))
        //add default plugins
        .add_plugins(DefaultPlugins)
        .add_asset::<TextureAssets>()
        .init_asset_loader::<TextureAssetsLoader>()
        //setup the game with cameras and everything
        .add_startup_system(setup_cameras.system())
        //add a state stage
        .add_stage_before(CoreStage::Update, STATE, StateStage::<GameState>::default())
        .on_state_enter(STATE, GameState::SetupStart, setup_start.system())
        .on_state_update(
            STATE,
            GameState::SetupStart,
            setup_continue_if_loaded.system(),
        )
        .on_state_enter(
            STATE,
            GameState::MainMenu,
            game_ui::systems::spawn_main_menu.system(),
        )
        .on_state_update(
            STATE,
            GameState::MainMenu,
            game_ui::systems::new_game_pressed.system(),
        )
        .on_state_enter(
            STATE,
            GameState::GenerateNewWorld,
            game_world::generation::systems::test_village_spawn_2.system(),
        )
        .on_state_update(
            STATE,
            GameState::InGame,
            ai::shared::systems::assign_target_to_bored_actors.system(),
        )
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .with(MainCamera)
        .spawn(UiCameraBundle::default());
    //println!("setup_cameras");
}
#[derive(Debug, Deserialize)]
pub struct KeyToHM(pub String);
#[derive(Debug, Deserialize)]
pub struct FileName(pub String);
#[derive(Debug, Deserialize)]
pub struct TextureAsset(pub FileName, pub KeyToHM);
#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct TextureAssets {
    pub assets: Vec<TextureAsset>,
}

#[derive(Default)]
pub struct TextureAssetsLoader;

impl AssetLoader for TextureAssetsLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<TextureAssets>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
pub struct TextureAssetsHandleHolder {
    handle: Handle<TextureAssets>,
    texture_handles: Vec<HandleUntyped>,
}
pub const KENNEY_SPRITES_FOLDER_PATH: &str = "Kenney_Retina";
pub const CUSTOM_SPRITES_FOLDER_PATH: &str = "Custom_Retina";

pub fn setup_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    //println!("setup_start");
    let handle: Handle<TextureAssets> = asset_server.load("texture_assets.ron");
    let mut texture_handles: Vec<HandleUntyped> = Vec::new();
    if let Ok(texture_handles_res) = asset_server.load_folder(KENNEY_SPRITES_FOLDER_PATH) {
        //println!("{:?}",texture_handles_res.iter().count());
        texture_handles = texture_handles_res;
    }
    if let Ok(texture_handles_res) = asset_server.load_folder(CUSTOM_SPRITES_FOLDER_PATH) {
        texture_handles.extend(texture_handles_res.into_iter());
    }

    commands.insert_resource(TextureAssetsHandleHolder {
        handle,
        texture_handles,
    });
}
#[derive(Debug, Clone)]
pub struct TextureSpawnInfo {
    pub handle: Handle<Texture>,
    pub rect: RectFromTransform,
}
pub struct SpriteHandleHashMapResource {
    pub map: HashMap<String, TextureSpawnInfo>,
}
pub fn setup_continue_if_loaded(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    texture_assets_handle_holder: Res<TextureAssetsHandleHolder>,
    mut state: ResMut<State<GameState>>,
    texture_assets: ResMut<Assets<TextureAssets>>,
    textures: ResMut<Assets<Texture>>,
) {
    let mut map = HashMap::<String, TextureSpawnInfo>::default();
    //if we loaded TextureAssets
    if let LoadState::Loaded = asset_server.get_group_load_state(
        texture_assets_handle_holder
            .texture_handles
            .iter()
            .map(|handle| handle.id),
    ) {
        if let Some(all_textures) = texture_assets.get(&texture_assets_handle_holder.handle) {
            for texture_asset in all_textures.assets.iter() {
                let mut path: String = KENNEY_SPRITES_FOLDER_PATH.to_owned();
                path.push_str("/");
                path.push_str(&*texture_asset.0 .0);
                let handle: Handle<Texture> = asset_server.get_handle(&*path);
                if let Some(texture) = textures.get(handle.to_owned()) {
                    map.insert(
                        texture_asset.1 .0.to_owned(),
                        TextureSpawnInfo {
                            handle,
                            rect: RectFromTransform {
                                width: texture.size.width,
                                height: texture.size.height,
                            },
                        },
                    );
                } else {
                    path = CUSTOM_SPRITES_FOLDER_PATH.to_owned();
                    path.push_str("/");
                    path.push_str(&*texture_asset.0 .0);
                    let handle: Handle<Texture> = asset_server.get_handle(&*path);
                    if let Some(texture) = textures.get(handle.to_owned()) {
                        map.insert(
                            texture_asset.1 .0.to_owned(),
                            TextureSpawnInfo {
                                handle,
                                rect: RectFromTransform {
                                    width: texture.size.width,
                                    height: texture.size.height,
                                },
                            },
                        );
                    }
                }
            }
            /*
            for (key,value) in map.iter(){
                println!("{:?},{:?}",key,value);
            }
            */
            commands
                .insert_resource(SpriteHandleHashMapResource { map })
                .remove_resource::<TextureAssetsHandleHolder>();
            state.set_next(GameState::MainMenu);
            //println!("setup_continue_if_loaded");
        }
    }
}
/*
let dir = "./assets/Kenney_Retina";
if let Ok(res) = std::fs::read_dir(dir.to_owned()) {
    for entry in res{
        //if the entry is fine
        if let Ok(entry) = entry {
            //if we can convert the filename to string
            if let Ok(file_name) = entry.file_name().into_string(){
                //if we want that file
                //println!("{:?}",string.to_owned());
                if let Some(item) =  all_textures.assets.iter().find(|texture_asset|{texture_asset.0.0 == file_name}){
                    let mut path_string = dir.to_owned();
                    path_string.push_str("/");
                    path_string.push_str(&file_name.to_owned());
                    //println!("{:?}",dir/*entry.path()*/);

                    map.insert(item.1.0.to_owned(), TextureSpawnInfo{
                        handle: asset_server.load(path_string),
                        rect:RectFromTransform::default()
                    });
                    //println!("{:?}",item);
                }

            }
            //println!("{:?}",entry);
        }
    }
     */
