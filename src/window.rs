pub mod window {
    use ggez::{
        Context, GameResult,
        event::{self, MouseButton},
        glam::*,
        graphics::{self, Color},
    };
    use rand::{seq::SliceRandom};
    use std::vec;

    use crate::physics::physics::Circle;

    pub const WINDOW_SIZE: Vec2 = vec2(1600., 800.);
    const CAN_PLACE: bool = false;
    const CAN_DRAG: bool = true;

    const DISTANCE_TO_VELOCITY_RATIO: f32 = 50.;

    pub struct MainState {
        circles: Vec<Circle>,
        is_placing: bool,
        place_position: Vec2,
        is_draging: bool,
        selected_circle: usize,
    }

    fn build_pyramid(size: i8, circles: &mut Vec<Circle>, position: Vec2) {
        let mut rng = rand::rng();

        let mut current = vec2(0.,0.);
        let start = position - vec2(size as f32 * 17.5, size as f32 * 17.5);

        let mut needed = vec![Color::BLACK];
        for _ in 0..7 {
            needed.push(Color::RED);
            needed.push(Color::BLUE);
        }
        needed.shuffle(&mut rng);

        for i in 1..=size {
            for _ in 1..=(size - i) {
                current.x += 0.5;
            }
            for _ in 1..=i {
                let mut circle = Circle::new(start + (current*35.), Vec2::ZERO);
                circle.color = needed.pop().expect("No Color left");
                circles.push(circle);
                current.x += 1.;
            }
            current.x = 0.;
            current.y -= 1.;
        }
    }

    impl MainState {
        pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
            let mut circles: Vec<Circle> = vec![];

            circles.push(Circle::new(vec2(WINDOW_SIZE.x/2.,WINDOW_SIZE.y/4.*3.), Vec2::ZERO));
            build_pyramid(5, &mut circles, vec2(WINDOW_SIZE.x/2., WINDOW_SIZE.y/2.));

            Ok(MainState {
                circles,
                is_placing: false,
                place_position: Vec2::ZERO,
                is_draging: false,
                selected_circle: 0,
            })
        }
    }

    impl event::EventHandler for MainState {
        fn update(&mut self, _ctx: &mut Context) -> GameResult {
            for prim in 0..self.circles.len() {
                for sec in (prim + 1)..self.circles.len() {
                    let (left, right) = self.circles.split_at_mut(sec);
                    let prim_circle = &mut left[prim];
                    let mut sec_circle = &mut right[0];

                    if prim_circle.is_colliding_with(&sec_circle) {
                        prim_circle.collide_with(&mut sec_circle);
                        //println!("COLLISION")
                    }
                }
            }

            for circle in &mut self.circles {
                circle.move_with_velocity(WINDOW_SIZE, _ctx.time.delta());
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
                let place_circle = Circle::new(self.place_position, vec2(0., 0.));
                place_circle.draw(&mut canvas, ctx);
            }

            if self.is_draging {
                self.circles[self.selected_circle].draw_outline(&mut canvas, ctx, 4., Color::RED);
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
                    // let mut rng = rand::rng();
                    // let radius = rng.random_range(10.0..40.0);

                    let id = (self.circles.len() + 1).to_string();
                    println!("Summon Circle with id: {}.", id);

                    let velocity = (vec2(x, y) - self.place_position) / DISTANCE_TO_VELOCITY_RATIO;
                    let circle = Circle::from_data(
                        self.place_position,
                        velocity,
                        15.,
                        Color::WHITE,
                        id,
                    );

                    self.circles.push(circle);

                    self.is_placing = false;
                } else if self.is_draging {
                    let circle = &mut self.circles[self.selected_circle];
                    circle.apply_force((vec2(x, y) - circle.position()) / DISTANCE_TO_VELOCITY_RATIO);

                    self.is_draging = false;
                    self.selected_circle = 0;
                } else {
                    if let Some(index) = self
                        .circles
                        .iter()
                        .position(|circle| circle.is_touching_point(vec2(x, y)))
                    {
                        self.selected_circle = index;
                        self.is_draging = CAN_DRAG;
                    } 
                    else {
                        self.is_placing = CAN_PLACE;
                        self.place_position = vec2(x, y);
                    }
                }
            }

            Ok(())
        }
    }
}