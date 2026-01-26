#![allow(clippy::unnecessary_wraps)]
pub mod physics;
pub mod window;

use ggez::{
    GameResult,
    conf::{FullscreenType, WindowMode},
    event::{self},
};

use crate::window::window::{MainState, WINDOW_SIZE};

pub fn main() -> GameResult {
    let window_mode = WindowMode {
        width: WINDOW_SIZE.x,
        height: WINDOW_SIZE.y,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None,
    };

    let cb = ggez::ContextBuilder::new("Physiks", "CodeTi").window_mode(window_mode);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}