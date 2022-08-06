use macroquad::{
    prelude::{rand, vec2, Rect, Vec2, RED},
    shapes::draw_rectangle,
    window::screen_width,
};

use crate::{BALL_SIZE, BALL_SPEED};

pub struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(position: Vec2) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rect.x += self.vel.x * delta_time * BALL_SPEED;
        self.rect.y += self.vel.y * delta_time * BALL_SPEED;
        self.detect_collision();
    }
    
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED)
    }


    fn detect_collision(&mut self) {
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }
}

// circle: Circle { x: (), y: (), r: (), },
