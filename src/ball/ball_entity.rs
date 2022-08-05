use macroquad::{
    prelude::{Circle, Vec2, RED},
    shapes::draw_circle,
    window::{screen_height, screen_width},
};

use crate::{BALL_SIZE, BALL_SPEED};

pub struct Ball {
    circle: Circle,
}

impl Ball {
    pub fn new(position: Vec2) -> Self {
        Self {
            circle: Circle::new(position.x, position.y, BALL_SIZE),
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, RED)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.circle.x += delta_time * BALL_SPEED;
        self.circle.y += delta_time * BALL_SPEED;
        self.detect_collision();
    }

    fn detect_collision(&mut self) {
        const LIMIT_HEIGHT: f32 = 0.618f32;

        if self.circle.x < 0f32 {
            self.circle.x = 0f32;
        }
        if self.circle.x > screen_width() - self.circle.r {
            self.circle.x = screen_width() - self.circle.r;
        }
        if self.circle.y < screen_height() * LIMIT_HEIGHT {
            self.circle.y = screen_height() * LIMIT_HEIGHT;
        }
        if self.circle.y > screen_height() - self.circle.r {
            self.circle.y = screen_height() - self.circle.r;
        }
    }
}

// circle: Circle { x: (), y: (), r: (), },
