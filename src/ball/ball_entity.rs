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
        const ACCELERATION: f32 = 0.3f32;
        let move_x: f32 = BALL_SPEED * (1f32 + ACCELERATION);
        let move_y: f32 = BALL_SPEED * (1f32 + ACCELERATION);


        // if self.circle.x < 0f32 {
        //     move_x += self.circle.x + BALL_SPEED;
        // }
        // if self.circle.x > screen_width() - self.circle.r {
        //     move_x -= screen_width() - self.circle.r;
        // }
        // if self.circle.y < 0f32  {
        //     move_y += self.circle.y + BALL_SPEED;
        // }
        // if self.circle.y > screen_height() - self.circle.r {
        //     move_y -= screen_height() - self.circle.r;
        // }
        
        self.circle.x += move_x * delta_time * BALL_SPEED;
        self.circle.y += move_y * delta_time * BALL_SPEED;
        self.detect_collision(move_x,move_y);
    }

    fn detect_collision(&mut self, move_x: f32, move_y: f32) {

        if self.circle.x < 0f32 {
            self.circle.x += move_x + 0f32;
        }
        if self.circle.x > screen_width() - self.circle.r {
            self.circle.x -= screen_width() - self.circle.r;
        }
        if self.circle.y < 0f32 {
            self.circle.y += move_y;
        }
        if self.circle.y > screen_height() - self.circle.r {
            self.circle.y -= screen_height() - self.circle.r;
        }
    }
}

// circle: Circle { x: (), y: (), r: (), },
