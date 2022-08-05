use crate::player::player_entity::Player;
use block::block_entity::Block;
use macroquad::prelude::*;

// start-region: -->      Mods
pub mod block;
pub mod player;
// __end-region: <--      Mods

// start-region: -->      CONSTANTS
pub const SIZE_FACTOR: f32 = 1.618f32;
pub const PLAYER_SPEED: f32 = 700f32;
pub const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
pub const BLOCK_SIZE: Vec2 = const_vec2!([75f32 * SIZE_FACTOR, 20f32 * SIZE_FACTOR]);
// __end-region: <--      CONSTANTS

////////////////////////////////////////////////////////////
// start-region: -->      MAIN
////////////////////////////////////////////////////////////
/// Main
///
/// # Docs Macroquad
/// https://macroquad.rs/docs/
#[macroquad::main("Breakout")]
async fn main() {
    let mut player: Player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();

    let (width, height) = (6, 6);
    let padding: f32 = 5f32;
    let total_block_size: Vec2 = BLOCK_SIZE + vec2(padding, padding);

    let board_position_start: Vec2 = vec2(
        (screen_width() - (total_block_size.x * width as f32)) / 2f32,
        50f32,
    );

    for i in 0..width * height {
        // generate looping value as --> 0,1,2,3,4,5,0,1,2,3,4,5,..
        let block_x: f32 = (i % width) as f32 * total_block_size.x;
        let block_y: f32 = (i / height) as f32 * total_block_size.y;

        blocks.push(Block::new(board_position_start + vec2(block_x, block_y)));
    }

    loop {
        player.update(get_frame_time());
        clear_background(VIOLET);

        player.draw();
        for block in blocks.iter() {
            block.draw();
        }

        next_frame().await
    }
}

////////////////////////////////////////////////////////////
// __end-region: <--      MAIN
////////////////////////////////////////////////////////////
