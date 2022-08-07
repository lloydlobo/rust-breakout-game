use crate::player::player_entity::Player;
use ball::ball_entity::Ball;
use block::block_entity::Block;
use macroquad::prelude::*;

// start-region: -->      Mods
pub mod ball;
pub mod block;
pub mod player;
// __end-region: <--      Mods

// start-region: -->      CONSTANTS
pub const SIZE_FACTOR: f32 = 1.618f32;
pub const PLAYER_SPEED: f32 = 700f32;
pub const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
pub const BLOCK_SIZE: Vec2 = const_vec2!([75f32 * SIZE_FACTOR, 20f32 * SIZE_FACTOR]);
pub const BALL_SIZE: f32 = 30f32;
pub const BALL_SPEED: f32 = 401f32;
// __end-region: <--      CONSTANTS


////////////////////////////////////////////////////////////
// start-region: -->      Collision Detection
////////////////////////////////////////////////////////////

pub fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect ) -> bool {
   if let Some (_intersection) = a.intersect(*b) {
      vel.y *= -1f32; 
      return true;
   }
    
   false
}

////////////////////////////////////////////////////////////
// __end-region: -->      Collision Detection
////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////
// start-region: -->      MAIN
////////////////////////////////////////////////////////////

/// Main
///
/// # Docs Macroquad
/// https://macroquad.rs/docs/
#[macroquad::main("Breakout")]
async fn main() {
    let (width, height) = (6, 6);
    let padding: f32 = 5f32;
    let total_block_size: Vec2 = BLOCK_SIZE + vec2(padding, padding);
    let board_pos_start: Vec2 = vec2(
        (screen_width() - (total_block_size.x * width as f32)) / 2f32,
        50f32,
    );

    let mut blocks: Vec<Block> = Vec::new();
    let mut player: Player = Player::new();
    let mut balls = Vec::new();

    let vec2_ball: Vec2 = vec2(screen_width() * 0.5f32, screen_height() * 0.5f32);

    for i in 0..width * height {
        let block_x: f32 = (i % width) as f32 * total_block_size.x;
        let block_y: f32 = (i / height) as f32 * total_block_size.y;
        blocks.push(Block::new(board_pos_start + vec2(block_x, block_y)));
    } // generate looping value as --> 0,1,2,3,4,5,0,1,2,3,4,5,..

    balls.push(Ball::new(vec2_ball));

    loop {
        if is_key_pressed(KeyCode::Space) {
            balls.push(Ball::new(vec2_ball));
        }
        player.update(get_frame_time());
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }

        for ball in balls.iter_mut() {
            resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
        }
        clear_background(VIOLET);

        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter_mut() {
            ball.draw();
        }
        player.draw();

        next_frame().await
    }
}

////////////////////////////////////////////////////////////
// __end-region: <--      MAIN
////////////////////////////////////////////////////////////
