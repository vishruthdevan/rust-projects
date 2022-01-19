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

struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}


impl Snake {
    pub fn new(x: 132, y: i32) → Snake {
        let mut body: LinkedList<Block> =LinkedList :: new();

        body.push_back(Block {
            x: x + 2,
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x,
            y,
        });
        Snake {
            direction: Direction :: Right,
            body,
            tail: None,
        }
    }
}