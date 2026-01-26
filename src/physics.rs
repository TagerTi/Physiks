pub mod physics {
    use std::time::Duration;

    use ggez::{
        Context,
        glam::*,
        graphics::{self, Color},
    };

    const AVERAGE_DENSITY: f32 = 5.514;
    const PI: f32 = 3.14159_26535;
    const FRICTION_MULTIPLYER: f32 = 0.996; // 0.995?
    // const RESTITUTION: f32 = 1.0;

    pub struct Circle {
        position: Vec2,
        velocity: Vec2,
        radius: f32,
        mass: f32,
        pub color: Color,
        pub tag: String,
    }

    impl Circle {
        pub fn new(position: Vec2, velocity: Vec2) -> Self {
            const DEFAULT_RADIUS: f32 = 15.;
            const DEFAULT_MASS: f32 = (PI * DEFAULT_RADIUS * DEFAULT_RADIUS) * AVERAGE_DENSITY;
            Self {
                position: position,
                velocity: velocity,
                radius: DEFAULT_RADIUS,
                mass: DEFAULT_MASS,
                color: Color::WHITE,
                tag: "not set".into(),
            }
        }

        pub fn from_data(
            position: Vec2,
            velocity: Vec2,
            radius: f32,
            color: Color,
            tag: String,
        ) -> Self {
            let mass = (PI * radius * radius) * AVERAGE_DENSITY;
            Self {
                position,
                velocity,
                radius,
                mass,
                color,
                tag,
            }
        }

        fn _reflect_from_edges(&mut self, screen_size: Vec2) {
            if self.position.x > screen_size.x - self.radius || self.position.x < self.radius {
                self.velocity.x *= -1.;
                self.position.x = self
                    .position
                    .x
                    .clamp(self.radius, screen_size.x - self.radius);
                //println!("Tag: {} reflected from x", self.tag);
            }

            if self.position.y > screen_size.y - self.radius || self.position.y < self.radius {
                self.velocity.y *= -1.;
                self.position.y = self
                    .position
                    .y
                    .clamp(self.radius, screen_size.y - self.radius);
                //println!("Tag: {} reflected from y", self.tag);
            }
        }

        fn _mirror_from_edges(&mut self, screen_size: Vec2) {
            if self.position.x > screen_size.x + self.radius {
                self.position.x -= screen_size.x + self.radius * 2.;
                //println!("Tag: {} moved to -x", self.tag);
            }
            if self.position.x < -self.radius {
                self.position.x += screen_size.x + self.radius * 2.;
                //println!("Tag: {} moved to +x", self.tag);
            }

            if self.position.y > screen_size.y + self.radius {
                self.position.y -= screen_size.y + self.radius * 2.;
                //println!("Tag: {} moved to -y", self.tag);
            }
            if self.position.y < -self.radius {
                self.position.y += screen_size.y + self.radius * 2.;
                //println!("Tag: {} moved to +y", self.tag);
            }
        }

        pub fn move_with_velocity(&mut self, screen_size: Vec2, dt: Duration) {
            //println!("{}", dt.as_secs_f32());
            self.position += self.velocity * dt.as_secs_f32() * 100.;
            self.velocity *= FRICTION_MULTIPLYER;

            self._reflect_from_edges(screen_size);

            // Nicht unbeding nötig. Nur zum Debugen

            if self.velocity.x.abs() > screen_size.x || self.velocity.y.abs() > screen_size.y {
                panic!("Speed of {} is to high to ...!", self.tag); // TODO: Find out what it does mean
            }

            if self.position.x > screen_size.x + self.radius * 2.
                || self.position.y > screen_size.y + self.radius * 2.
                || self.position.x < -self.radius * 2.
                || self.position.y < -self.radius * 2.
            {
                panic!("Position of {} is strange! It is out of screen!", self.tag);
            }
        }

        pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &Context) {
            let t = 0.01 * self.radius;

            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                vec2(0., 0.),
                self.radius,
                t,
                self.color,
            );

            canvas.draw(&circle.expect("AAA"), self.position);
        }

        pub fn draw_outline(&self, canvas: &mut graphics::Canvas, ctx: &Context, width: f32, color: Color) {
            let t = 0.01 * self.radius;

            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::stroke(width),
                vec2(0., 0.),
                self.radius,
                t,
                color,
            );

            canvas.draw(&circle.expect("AAA"), self.position);
        }

        pub fn is_colliding_with(&self, other: &Circle) -> bool {
            let distance = (other.position - self.position).length(); // Falls langsam, weg von sqrt!
            distance < self.radius + other.radius
        }

        pub fn collide_with(&mut self, other: &mut Circle) {
            let line_of_impact = other.position - self.position;
            let distance = line_of_impact.length();

            let collision_normal = line_of_impact.normalize();

            // Seperating Circles
            let overlaping_depth = distance - (self.radius + other.radius);
            self.position += collision_normal * overlaping_depth / 2.;
            other.position -= collision_normal * overlaping_depth / 2.;

            // Random berechnungen
            let mass_sum = self.mass + other.mass;
            let vel_diff = other.velocity - self.velocity;

            // Zaehler und Naenner von Kreis A von der Formel, die aus einem Bruch besteht
            let zaehler = 2. * vel_diff.dot(line_of_impact);
            let naenner = mass_sum * distance * distance;

            // Kreis A
            let mut delta_vel_a = line_of_impact.clone();
            delta_vel_a *=  other.mass * zaehler / naenner;
            self.velocity += delta_vel_a;

            // Kreis B
            let mut delta_vel_b = line_of_impact.clone();
            delta_vel_b *= -self.mass * zaehler / naenner;
            other.velocity += delta_vel_b;
        }

        pub fn is_touching_point(&self, point: Vec2) -> bool {
            (point - self.position).length() < self.radius
        }

        pub fn apply_force(&mut self, force: Vec2) {
            self.velocity += force;
        }

        pub fn position(&self) -> Vec2 {
            self.position
        }
        pub fn velocity(&self) -> Vec2 {
            self.velocity
        }
    }
}
