//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use std::vec;
use rand::Rng;
use ggez::{ Context, GameResult, conf::{Backend, Conf, FullscreenType, WindowMode, WindowSetup}, event::{self, MouseButton}, glam::*, graphics::{self, Color}, input::keyboard::KeyCode };

const WINDOW_SIZE: Vec2 = vec2(800.,600.);

struct Circle {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
    tag: String,
}

impl Circle {
    fn move_with_velocity(&mut self, screen_size: Vec2) { // Speed should not be faster than screen size.
        self.position += self.velocity;

        if self.position.x > screen_size.x {self.position.x -= screen_size.x + self.radius*2.}
        if self.position.x < -self.radius {self.position.x += screen_size.x + self.radius*2.}

        if self.position.y > screen_size.y + self.radius {self.position.y -= screen_size.y + self.radius*2.}
        if self.position.y < -self.radius {self.position.y += screen_size.y + self.radius*2.}

        if self.velocity.x.abs() > screen_size.x || 
           self.velocity.y.abs() > screen_size.y 
        {
            panic!("Speed is to high to ...!") // TODO: Find out what it does mean
        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas, ctx: &Context) {
        let t = 0.01*self.radius;

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0.,0.),
            self.radius,
            t,
            self.color
        );

        canvas.draw(&circle.expect("AAA"), self.position);
    }
}

struct MainState {
    circles: Vec<Circle>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut circles: Vec<Circle> = vec![];
        
        circles.push(Circle { position: vec2(10.,380.0), velocity: vec2(1.,0.), radius: 15., color: Color::WHITE, tag: "first".into()});

        Ok(MainState { circles })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for circle in &mut self.circles {
            circle.move_with_velocity(WINDOW_SIZE);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for circle in &mut self.circles {
            circle.draw(&mut canvas, ctx);
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        if button == MouseButton::Left {
            let mut rng = rand::rng();

            let velocity = vec2(rng.random_range(-1.0..1.0),rng.random_range(-1.0..1.0));
            let circle = Circle { position: vec2(x,y), velocity, radius: 15., color: Color::WHITE, tag: "summoned".into()};
            
            self.circles.push(circle);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        

        Ok(())
    }
}

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