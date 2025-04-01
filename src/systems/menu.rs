use bevy::prelude::*;
use crate::resources::GameState;
use crate::components::MainCamera;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct MenuCamera; // ✅ Tag for the menu camera

/// Spawns the main menu UI with a Play button
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ✅ Spawn menu camera and tag it
    commands.spawn((
        Camera2dBundle::default(),
        MenuCamera,
    ));

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        },
        MainMenuUI,
    ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::GRAY.into(),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

/// Handles clicking the Play button
pub fn handle_play_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    menu_camera_query: Query<Entity, With<MenuCamera>>, // ✅ To remove menu camera
    ui_query: Query<Entity, With<MainMenuUI>>,           // ✅ Optional: to remove the menu UI
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // ✅ Despawn menu camera
                for cam_entity in menu_camera_query.iter() {
                    commands.entity(cam_entity).despawn();
                }

                // ✅ Despawn the entire menu UI
                for ui_entity in ui_query.iter() {
                    commands.entity(ui_entity).despawn_recursive();
                }

                next_state.set(GameState::Running);
                println!("▶️ Play button clicked!");
            }
            Interaction::Hovered => {
                *color = Color::DARK_GRAY.into();
            }
            Interaction::None => {
                *color = Color::GRAY.into();
            }
        }
    }
}
