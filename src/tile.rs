use macroquad::{
    color::{BLUE, GRAY, WHITE},
    math::Vec2,
    shapes::draw_rectangle,
    texture::{draw_texture_ex, DrawTextureParams},
};

use crate::textures::Textures;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TileState {
    Empty,
    FlaggedUserOne,
    FlaggedUserTwo,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub state: TileState,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            state: TileState::Empty,
        }
    }

    pub fn draw(&self, x: f32, y: f32, size: f32, textures: &Textures) {
        let color = match &self.state {
            TileState::Empty => GRAY,
            TileState::FlaggedUserOne => BLUE,
            TileState::FlaggedUserTwo => BLUE,
        };

        draw_rectangle(x, y, size, size, color);

        if let Some(texture) = match &self.state {
            TileState::Empty => None,
            TileState::FlaggedUserOne => Some(&textures.one),
            TileState::FlaggedUserTwo => Some(&textures.two),
        } {
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(size)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn set_tile_state(&mut self, state: TileState) {
        self.state = state;
    }
}
