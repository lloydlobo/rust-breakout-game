use macroquad::prelude::*;

use crate::{PLAYER_SIZE, PLAYER_SPEED};

////////////////////////////////////////////////////////////
// start-region: -->      PLAYER
////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
pub struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    /// Updates the player controls
    /// - delta_time: f32 - used to move player equally fast regardless of fps of the game
    pub fn update(&mut self, delta_time: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        let y_move = match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        self.rect.x += x_move * delta_time * PLAYER_SPEED;
        self.rect.y += y_move * delta_time * PLAYER_SPEED.sqrt();

        self.detect_collision();
    }

    fn detect_collision(&mut self) {
        const LIMIT_HEIGHT: f32 = 0.618f32;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
        if self.rect.y < screen_height() * LIMIT_HEIGHT {
            self.rect.y = screen_height() * LIMIT_HEIGHT;
        }
        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, PURPLE);
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

////////////////////////////////////////////////////////////
// __end-region: <--      PLAYER
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use macroquad::miniquad::{date::now, native::linux_x11::libx11::Time};

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1f32 + 1f32, 2f32);
    }

    #[test]
    fn test_player_entity() {
        assert_eq!(1, 1);
    }

    #[test]
    fn it_gets_frame_time() {
        pub fn get_time() -> f64 {
            let start_time_duration = std::time::SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|e| panic!("{}", e));
        let now = start_time_duration.as_secs_f64();

            miniquad::date::now() - now
        }
    }
}

// pub fn update_control_classic(&mut self, delta_time: f32) {
//     let mut x_move = 0f32;
//     if is_key_down(KeyCode::Left) {
//         x_move -= 1f32;
//     }
//     if is_key_down(KeyCode::Right) {
//         x_move += 1f32;
//     }
// }
