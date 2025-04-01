use bevy::prelude::*;
use crate::resources::{CoinWallet, CurrentSkin};

#[derive(Component)]
pub struct ShopUI;

#[derive(Component)]
pub struct BuyButton;

pub fn shop_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    wallet: Res<CoinWallet>,
    query: Query<Entity, With<ShopUI>>,
) {
    if !query.is_empty() {
        return;
    }

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(150.0),
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                right: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        },
        ShopUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("Coins: {}", wallet.coins),
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        ));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(180.0),
                    height: Val::Px(45.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::GRAY.into(),
                ..default()
            },
            BuyButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Buy Green Skin (1🪙)",
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

pub fn handle_buy_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BuyButton>),
    >,
    mut skin: ResMut<CurrentSkin>,
    mut wallet: ResMut<CoinWallet>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let cost = 1;
                if wallet.coins >= cost {
                    wallet.coins -= cost;
                    skin.color = Color::rgb(0.2, 0.8, 0.2);
                    println!("✅ Bought green skin! Coins left: {}", wallet.coins);
                } else {
                    println!("❌ Not enough coins!");
                }
                *color = Color::DARK_GRAY.into();
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
            }
            Interaction::None => {
                *color = Color::GRAY.into();
            }
        }
    }
}
