use crate::common::core::marker_components::*;
use crate::GameState;
use bevy::prelude::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with(UiRoot)
        .with_children(|root_node| {
            root_node
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    material: materials.add(Color::BLACK.into()),
                    ..Default::default()
                })
                .with_children(|main_menu_container| {
                    main_menu_container
                        .spawn(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "System World",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .spawn(ButtonBundle {
                            style: Style {
                                margin: Rect {
                                    top: Val::Px(150.),
                                    ..Default::default()
                                },
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            material: materials.add(Color::GRAY.into()),
                            ..Default::default()
                        })
                        .with(NewGameButton)
                        .with_children(|button_parent| {
                            button_parent.spawn(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "New Game",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
        });
}

pub fn new_game_pressed(
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    root_node_query: Query<Entity, With<UiRoot>>,
    button_query: Query<&Interaction, With<NewGameButton>>,
) {
    if let Some(interaction) = button_query.iter().next() {
        match interaction {
            Interaction::Clicked => {
                if let Some(entity) = root_node_query.iter().next() {
                    commands.despawn_recursive(entity);
                }
                state.set_next(GameState::GenerateNewWorld).unwrap();
            }
            _ => {}
        }
    }
}
