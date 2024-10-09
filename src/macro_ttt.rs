use macroquad::{
    color::RED,
    input::MouseButton,
    text::{draw_text, measure_text},
    window::{screen_height, screen_width},
};

use crate::{
    mouse::get_pressed_mouse_position,
    position::Position,
    textures::Textures,
    tile::{Tile, TileState},
};

const ROWS: i32 = 3;
const COLS: i32 = 3;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Won,
    NoMovesLeft,
}

enum Player {
    One,
    Two,
}

pub struct MacroTTT {
    pub tiles: Vec<Tile>,
    pub state: GameState,
    textures: Textures,
    current_player: Player,
    player_won_string: String,
}

impl MacroTTT {
    pub async fn new() -> Self {
        let tiles = Self::create_tiles(ROWS, COLS);
        let textures = Textures::load().await;

        Self {
            tiles,
            state: GameState::Playing,
            textures,
            current_player: Player::One,
            player_won_string: String::new(),
        }
    }

    pub fn draw(&self) {
        match self.state {
            GameState::Playing => {
                let tile_size = self.get_tile_size();

                for i in 0..COLS {
                    for j in 0..ROWS {
                        let index = self.get_index(&Position::new(i, j));
                        let tile = &self.tiles[index];
                        tile.draw(
                            i as f32 * tile_size,
                            j as f32 * tile_size,
                            tile_size - 1.0,
                            &self.textures,
                        );
                    }
                }
            }
            GameState::Won => {
                let text_dimensions = measure_text(&self.player_won_string, None, 50, 1.0);
                draw_text(
                    &self.player_won_string,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );

                let restart_text = "Press Space to Restart";
                let restart_text_dimensions = measure_text(restart_text, None, 30, 1.0);
                draw_text(
                    restart_text,
                    screen_width() / 2.0 - restart_text_dimensions.width / 2.0,
                    (screen_height() / 2.0) + restart_text_dimensions.height + 10.0,
                    30.0,
                    RED,
                );
            }
            GameState::NoMovesLeft => {
                let text = "NO MOVES LEFT!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );

                let restart_text = "Press Space to Restart";
                let restart_text_dimensions = measure_text(restart_text, None, 30, 1.0);
                draw_text(
                    restart_text,
                    screen_width() / 2.0 - restart_text_dimensions.width / 2.0,
                    (screen_height() / 2.0) + restart_text_dimensions.height + 10.0,
                    30.0,
                    RED,
                );
            }
        }
    }

    pub fn handle_input(&mut self) {
        if let Some(pos) = get_pressed_mouse_position(MouseButton::Left) {
            self.make_move(pos)
        }
    }

    fn make_move(&mut self, pos: Position<f32>) {
        let pos = match self.resolve_tile_position(&pos) {
            Some(value) => value,
            None => return,
        };

        let index = self.get_index(&pos);
        let tile = &mut self.tiles[index];

        if tile.state == TileState::Empty {
            match &self.current_player {
                Player::One => {
                    tile.set_tile_state(TileState::FlaggedUserOne);
                }
                Player::Two => {
                    tile.set_tile_state(TileState::FlaggedUserTwo);
                }
            }

            self.change_current_player();
        }

        // Check if any user has won
        match self.check_win() {
            Some(player) => {
                self.player_won(player);
            }
            None => {
                if self.is_board_full() {
                    self.no_moves_left();
                }
            }
        }
    }

    fn player_won(&mut self, player: Player) {
        self.state = GameState::Won;
        match player {
            Player::One => self.player_won_string = String::from("Player 1 Wins"),
            Player::Two => self.player_won_string = String::from("Player 2 Wins"),
        }
    }

    fn no_moves_left(&mut self) {
        self.state = GameState::NoMovesLeft;
    }

    // Check if there are no possible moves left
    fn is_board_full(&self) -> bool {
        self.tiles.iter().all(|tile| tile.state != TileState::Empty)
    }

    // Check if someone won, returns a player if either of them won or none if no one won yet
    fn check_win(&self) -> Option<Player> {
        // Check rows
        for row in 0..3 {
            let first = self.tiles[row * 3].state;
            if first != TileState::Empty
                && first == self.tiles[row * 3 + 1].state
                && first == self.tiles[row * 3 + 2].state
            {
                return match first {
                    TileState::FlaggedUserOne => Some(Player::One),
                    TileState::FlaggedUserTwo => Some(Player::Two),
                    _ => None,
                };
            }
        }

        // Check columns
        for col in 0..3 {
            let first = self.tiles[col].state;
            if first != TileState::Empty
                && first == self.tiles[col + 3].state
                && first == self.tiles[col + 6].state
            {
                return match first {
                    TileState::FlaggedUserOne => Some(Player::One),
                    TileState::FlaggedUserTwo => Some(Player::Two),
                    _ => None,
                };
            }
        }

        // Check diagonals
        let center = self.tiles[4].state;
        if center != TileState::Empty {
            // Top-left to bottom-right
            if center == self.tiles[0].state && center == self.tiles[8].state {
                return match center {
                    TileState::FlaggedUserOne => Some(Player::One),
                    TileState::FlaggedUserTwo => Some(Player::Two),
                    _ => None,
                };
            }
            // Top-right to bottom-left
            if center == self.tiles[2].state && center == self.tiles[6].state {
                return match center {
                    TileState::FlaggedUserOne => Some(Player::One),
                    TileState::FlaggedUserTwo => Some(Player::Two),
                    _ => None,
                };
            }
        }

        None
    }

    fn change_current_player(&mut self) {
        match self.current_player {
            Player::One => self.current_player = Player::Two,
            Player::Two => self.current_player = Player::One,
        }
    }

    // Resolves which tile the user clicked on, it returns an option because
    // it's possible the user clicked outside the board.
    fn resolve_tile_position(&self, pos: &Position<f32>) -> Option<Position<i32>> {
        let tile_size = self.get_tile_size();
        let divided_pos = pos.div(tile_size);

        let result = divided_pos.into();

        if self.within_bounds(&result) {
            Some(result)
        } else {
            None
        }
    }

    fn within_bounds(&self, pos: &Position<i32>) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < COLS && pos.y < ROWS
    }

    fn get_index(&self, pos: &Position<i32>) -> usize {
        (COLS * pos.y + pos.x) as usize
    }

    fn get_tile_size(&self) -> f32 {
        let width = screen_width() / COLS as f32;
        let height = screen_height() / ROWS as f32;

        width.min(height)
    }

    fn create_tiles(rows: i32, cols: i32) -> Vec<Tile> {
        vec![Tile::new(); (rows * cols) as usize]
    }
}
