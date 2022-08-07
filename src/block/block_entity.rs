use macroquad::{
    prelude::{Rect, Vec2, GOLD},
    shapes::draw_rectangle,
};

use crate::BLOCK_SIZE;

pub struct Block {
    rect: Rect,
    lives: i32,
}

impl Block {
    pub fn new(position: Vec2) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 1,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GOLD);
    }
}
