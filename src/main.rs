use macro_ttt::MacroTTT;
use macroquad::prelude::*;

mod macro_ttt;
mod mouse;
mod position;
mod textures;
mod tile;

#[macroquad::main("MacroTTT")]
async fn main() {
    set_pc_assets_folder("assets");
    let mut game = MacroTTT::new().await;

    loop {
        clear_background(BLACK);

        game.handle_input();
        game.draw();

        // Reset the game when not playing and space is pressed
        if game.state != macro_ttt::GameState::Playing && is_key_pressed(KeyCode::Space) {
            game = MacroTTT::new().await;
        }

        next_frame().await;
    }
}
