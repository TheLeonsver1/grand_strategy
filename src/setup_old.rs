pub mod common;
pub mod game_generation;
pub mod game_ui;
pub mod game_world;

use bevy::{asset::{AssetPath, AssetPathId, LoadState}, prelude::*, utils::HashMap};
use common::{core::marker_components::MainCamera, map::data_components::RectFromTransform};
use game_world::{ai, ai_hierarchy, buildings, settlements};
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
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(State::new(GameState::MainMenu))
        //add default plugins
        .add_plugins(DefaultPlugins)
        //setup the game with cameras and everything
        .add_startup_system(setup_cameras.system())
        //add a state stage
        .add_stage_before(CoreStage::Update, STATE, StateStage::<GameState>::default())
        .on_state_enter(
            STATE,
            GameState::SetupStart,
            setup_add_temp_resources.system(),
        )
        .on_state_enter(
            STATE,
            GameState::SetupLoadAssets,
            setup_load_assets.system(),
        )
        .on_state_update(
            STATE,
            GameState::SetupLoadAssets,
            setup_check_loaded.system(),
        )
        .on_state_enter(
            STATE,
            GameState::SetupFinish,
            setup_remove_temp_resources_and_add_permanent_resources.system(),
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
            game_generation::systems::test_village_spawn.system(),
        )
        .on_state_update(
            STATE,
            GameState::InGame,
            ai::systems::assign_target_to_bored_actors.system(),
        )
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .with(MainCamera)
        .spawn(UiCameraBundle::default());
}
#[derive(Default)]
struct KenneyTexturesResource(Vec<HandleUntyped>, bool);
#[derive(Default)]
struct CustomTexturesResource(Vec<HandleUntyped>, bool);
#[derive(Default)]
struct FontsResource(Vec<HandleUntyped>, bool);

fn setup_add_temp_resources(mut commands: Commands) {
    commands
        .insert_resource(KenneyTexturesResource::default())
        .insert_resource(CustomTexturesResource::default())
        .insert_resource(FontsResource::default());
}
fn setup_load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut kenneyTextures: ResMut<KenneyTexturesResource>,
    mut customTextures: ResMut<CustomTexturesResource>,
    mut fonts: ResMut<FontsResource>,
) {
    if let Ok(texture_handles) = asset_server.load_folder("Kenney_Retina") {
        kenneyTextures.0 = texture_handles;
    }
    if let Ok(texture_handles) = asset_server.load_folder("Custom_Retina") {
        customTextures.0 = texture_handles;
    }
    if let Ok(font_handles) = asset_server.load_folder("Fonts") {
        fonts.0 = font_handles;
    }
}
fn setup_check_loaded(
    asset_server: Res<AssetServer>,
    mut kenney_textures: ResMut<KenneyTexturesResource>,
    mut custom_textures: ResMut<CustomTexturesResource>,
    mut fonts: ResMut<FontsResource>,
) {
    let mut counter = 0;
    //if we haven't loaded still in the last frame
    if !kenney_textures.1 {
        //check if it was loaded by this frame
        if let LoadState::Loaded =
            asset_server.get_group_load_state(kenney_textures.0.iter().map(|handle| handle.id))
        {
            counter += 1;
            //and make sure we don't check again
            kenney_textures.1 = true;
        }
    } else {
        //we already checked and it's loaded so ready to go
        counter += 1;
    }
    if !custom_textures.1 {
        if let LoadState::Loaded =
            asset_server.get_group_load_state(custom_textures.0.iter().map(|handle| handle.id))
        {
            counter += 1;
            custom_textures.1 = true;
        }
    } else {
        counter += 1;
    }
    if !custom_textures.1 {
        if let LoadState::Loaded =
            asset_server.get_group_load_state(fonts.0.iter().map(|handle| handle.id))
        {
            counter += 1;
            fonts.1 = true;
        }
    } else {
        counter += 1;
    }
    //all three asset types are loaded, awesome
    if counter == 3 {}
}
pub struct TextureInfo(Handle<Texture>,RectFromTransform);
pub struct TextureInfoHashMapResource{
    pub map:HashMap::<AssetPathId, TextureInfo>
}
fn setup_remove_temp_resources_and_add_permanent_resources(
    mut commands: Commands,
    asset_server:Res<AssetServer>,
    textures: Res<Assets<Texture>>,
    fonts: Res<Assets<Font>>,
    mut temp_kenney:ResMut<KenneyTexturesResource>,
    mut temp_custom_textures: ResMut<CustomTexturesResource>,
    mut temp_fonts: ResMut<FontsResource>,

) {
    let mut textures_hash_map = HashMap::<AssetPathId, TextureInfo>::default();
    for handle in temp_kenney.0.iter(){
        if let Some(texture) = textures.get(handle){
            if let Some(path) = asset_server.get_handle_path(handle){
                textures_hash_map.insert(path.get_id(),TextureInfo(textures.get_handle(handle),RectFromTransform{
                    width: texture.size.width,
                    height:texture.size.height}));
            }
            
        }
    }

    let fonts_hash_map= HashMap::<String, Handle<Font>>::default();

    commands
        .insert_resource(TextureInfoHashMapResource{
            map:textures_hash_map
        })
        .remove_resource::<KenneyTexturesResource>()
        .remove_resource::<CustomTexturesResource>()
        .remove_resource::<FontsResource>();
}
