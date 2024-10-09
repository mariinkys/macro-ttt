use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};

use crate::position::Position;

pub fn get_pressed_mouse_position(btn: MouseButton) -> Option<Position<f32>> {
    if is_mouse_button_pressed(btn) {
        let (x, y) = mouse_position();
        return Some(Position::new(x, y));
    }
    None
}
