
pub mod window {
    use std::{vec};
    use ggez::{ Context, GameResult, event::{self, MouseButton}, glam::*, graphics::{self, Color} };

    use crate::physics::physics::Circle;

    pub const WINDOW_SIZE: Vec2 = vec2(1000.,800.);

    pub struct MainState {
        circles: Vec<Circle>,
        is_placing: bool,
        place_position: Vec2,
        is_draging: bool,
        selected_circle: usize,
    }

    impl MainState {
        pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
            let circles: Vec<Circle> = vec![];

            Ok(MainState { circles, is_placing: false, place_position: Vec2::ZERO, is_draging: false, selected_circle: 0 })
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
                    let circle = Circle::from_data(self.place_position, velocity, 15., Color::WHITE, id);
                    
                    self.circles.push(circle);

                    self.is_placing = false;
                }
                else if self.is_draging {
                    let circle = &mut self.circles[self.selected_circle];
                    circle.apply_force((vec2(x,y) - circle.position()) / 100.);

                    self.is_draging = false;
                    self.selected_circle = 0;
                }
                else {
                    if let Some(index) = self.circles.iter().position(|circle| circle.is_touching_point(vec2(x, y))) {
                        self.selected_circle = index;
                        self.is_draging = true;
                    } 
                    else {
                        self.is_placing = true;
                        self.place_position = vec2(x,y);
                    }
                }
            }

            Ok(())
        }
    }
}