//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use core::time;
use std::{vec};
use ggez::{ Context, GameResult, conf::{FullscreenType, WindowMode}, event::{self, MouseButton}, glam::*, graphics::{self, Color} };

const WINDOW_SIZE: Vec2 = vec2(1000.,800.);
const AVERAGE_DENSITY: f32 = 5.514;
const PI: f32 = 3.14159_26535;

struct Circle {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
    tag: String,
}

impl Circle {
    fn mass(&self) -> f32 {
        (PI * self.radius*self.radius) * AVERAGE_DENSITY
    }

    fn new(position: Vec2, velocity: Vec2) -> Self{
        Self {
            position: position,
            velocity: velocity,
            radius: 15.,
            color: Color::WHITE,
            tag: "not set".into(),
        }
    }

    fn move_with_velocity(&mut self, screen_size: Vec2) { // Speed should not be faster than screen size.
        self.position += self.velocity;

        if self.position.x > screen_size.x + self.radius {
            self.position.x -= screen_size.x + self.radius*2.;
            println!("Tag: {} moved to -x", self.tag);
        }
        if self.position.x < -self.radius {
            self.position.x += screen_size.x + self.radius*2.;
            println!("Tag: {} moved to +x", self.tag);
        }

        if self.position.y > screen_size.y + self.radius {
            self.position.y -= screen_size.y + self.radius*2.; 
            println!("Tag: {} moved to -y", self.tag);
        }
        if self.position.y < -self.radius {
            self.position.y += screen_size.y + self.radius*2.;
            println!("Tag: {} moved to +y", self.tag);
        }
        

        // Nicht unbeding nötig. Nur zum Debugen

        if self.velocity.x.abs() > screen_size.x || 
           self.velocity.y.abs() > screen_size.y 
        {
            panic!("Speed of {} is to high to ...!", self.tag); // TODO: Find out what it does mean
        }

        if self.position.x > screen_size.x + self.radius*2. || self.position.y > screen_size.y + self.radius*2. ||
            self.position.x < -self.radius*2. || self.position.y < -self.radius*2. 
        {    
            panic!("Position of {} is strange! It is out of screen!", self.tag);
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

    fn collides(&self, other: &Circle) -> bool {
        let distance = (other.position - self.position).length();
        distance < self.radius + other.radius
    }

    fn collide_with(&mut self, other: &mut Circle) {
        // Calculating things
        let self_energy = self.velocity.length() * self.mass();
        let other_energy = other.velocity.length() * other.mass();

        let collision_normal = (other.position - self.position).normalize();

        let energy = self_energy + other_energy;
        let velocity = (self.velocity + other.velocity).length();

        // Setting new Velocity
        let self_new_vel = -collision_normal * velocity * other_energy / energy;
        let other_new_vel = collision_normal * velocity * self_energy / energy;

        self.velocity = self_new_vel;
        other.velocity = other_new_vel;

        // Seperating Circles
        let overlaping_factor = (other.position - self.position).length() - (self.radius + other.radius);
        self.position += collision_normal * overlaping_factor/2.;
        other.position -= collision_normal * overlaping_factor/2.;
    }
}

struct MainState {
    circles: Vec<Circle>,
    is_placing: bool,
    place_position: Vec2,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut circles: Vec<Circle> = vec![];
        
        // circles.push(
        //     Circle { 
        //         position: vec2(10.,380.0), 
        //         velocity: vec2(1.,0.), 
        //         radius: 15., 
        //         color: Color::WHITE, 
        //         tag: "first".into()
        //     }
        // );

        Ok(MainState { circles, is_placing: false, place_position: Vec2::ZERO })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for circle in &mut self.circles {
            circle.move_with_velocity(WINDOW_SIZE);
        }

        for prim in 0..self.circles.len() {
            for sec in (prim+1)..self.circles.len() {
                let (left, right) = self.circles.split_at_mut(sec);
                let prim_circle = &mut left[prim];
                let mut sec_circle = &mut right[0];

                if prim_circle.collides(&sec_circle) {
                    prim_circle.collide_with(&mut sec_circle);
                    println!("COLLISION")
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for circle in &mut self.circles {
            circle.draw(&mut canvas, ctx);
        }

        if self.is_placing {
            let place_circle = Circle::new(self.place_position, vec2(0.,0.));
            place_circle.draw(&mut canvas, ctx);
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
            if self.is_placing {
                let id = (self.circles.len()+1).to_string();
                println!("Summon Circle with id: {}.", id);

                let velocity = (vec2(x,y) - self.place_position) / 100.;
                let circle = Circle { position: self.place_position, velocity, radius: 15., color: Color::WHITE, tag: id};
                
                self.circles.push(circle);

                self.is_placing = false;
            }
            else {
                self.is_placing = true;
                self.place_position = vec2(x,y);
            }
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _input: ggez::input::keyboard::KeyInput,
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