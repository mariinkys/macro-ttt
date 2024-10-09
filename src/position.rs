use std::ops::{Add, Div, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub fn sub(&self, other: &Position<T>) -> Self
    where
        T: Sub<T, Output = T> + Copy,
    {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[allow(dead_code)]
    pub fn add(&self, other: &Position<T>) -> Self
    where
        T: Add<T, Output = T> + Copy,
    {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn div(&self, val: T) -> Self
    where
        T: Div<T, Output = T> + Copy,
    {
        Position {
            x: self.x / val,
            y: self.y / val,
        }
    }
}

impl From<Position<f32>> for Position<i32> {
    fn from(value: Position<f32>) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}
