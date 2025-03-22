use bevy::prelude::*;
use crate::resources::{CoinWallet, CurrentSkin};

pub fn open_shop(
    keyboard_input: Res<Input<KeyCode>>,
    mut wallet: ResMut<CoinWallet>,
    mut skin: ResMut<CurrentSkin>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        println!("🛍️ Opening Shop...");

        let cost = 1;
        let new_color = Color::rgb(0.2, 0.8, 0.2); // Green skin

        if wallet.coins >= cost {
            wallet.coins -= cost;
            skin.color = new_color;
            println!("✅ Purchased green skin! Coins left: {}", wallet.coins);
        } else {
            println!("❌ Not enough coins. You need {} but have {}", cost, wallet.coins);
        }
    }
}
