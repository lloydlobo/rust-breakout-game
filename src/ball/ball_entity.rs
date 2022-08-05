use macroquad::{
    prelude::{Circle, Vec2, RED},
    shapes::draw_circle,
};

use crate::BALL_SIZE;

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
}

// circle: Circle { x: (), y: (), r: (), },
