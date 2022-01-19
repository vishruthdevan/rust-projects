use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use draw::draw_block;

const SNAKE_COLOUR: Color = [0.0, 0.80, 0.0, 1.0];

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
