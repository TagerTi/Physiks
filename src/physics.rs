

pub mod physics {
    use ggez::{ Context, glam::*, graphics::{self, Color} };    

    const AVERAGE_DENSITY: f32 = 5.514;
    const PI: f32 = 3.14159_26535;
    const FRICTION_MULTIPLYER: f32 = 1.; // 0.995?
    const RESTITUTION: f32 = 0.5;

    pub struct Circle {
        position: Vec2,
        velocity: Vec2,
        radius: f32,
        color: Color,
        tag: String,
    }

    impl Circle {
        pub(crate) fn mass(&self) -> f32 {
            (PI * self.radius*self.radius) * AVERAGE_DENSITY
        }

        pub fn new(position: Vec2, velocity: Vec2) -> Self{
            Self {
                position: position,
                velocity: velocity,
                radius: 15.,
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
                Self {
                    position,
                    velocity,
                    radius,
                    color,
                    tag,
                }
        }

        fn _reflect_from_edges(&mut self, screen_size: Vec2) {
            if self.position.x > screen_size.x - self.radius || self.position.x < self.radius {
                self.velocity.x *= -1.;
                self.position.x = self.position.x.clamp(self.radius, screen_size.x - self.radius);
                println!("Tag: {} reflected from x", self.tag);
            }

            if self.position.y > screen_size.y - self.radius || self.position.y < self.radius {
                self.velocity.y *= -1.;
                self.position.y = self.position.y.clamp(self.radius, screen_size.x - self.radius); 
                println!("Tag: {} reflected from y", self.tag);
            }
        }

        fn _mirror_from_edges(&mut self, screen_size: Vec2) {
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
        }

        pub fn move_with_velocity(&mut self, screen_size: Vec2) {
            self.position += self.velocity;
            self.velocity *= FRICTION_MULTIPLYER;

            self._reflect_from_edges(screen_size);
            

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

        pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &Context) {
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

        pub fn collides(&self, other: &Circle) -> bool {
            let distance = (other.position - self.position).length(); // Falls langsam, weg von sqrt!
            distance < self.radius + other.radius
        }

        pub fn collide_with_old(&mut self, other: &mut Circle) { // Old and wrong
            // Calculating things
            let self_energy = ( self.mass() * self.velocity.length()*self.velocity.length() ) / 2.;
            let other_energy = ( other.mass() * other.velocity.length()*other.velocity.length() ) / 2.;

            let collision_normal = (other.position - self.position).normalize();

            let energy = self_energy + other_energy;
            let velocity = (self.velocity + other.velocity).length();

            // Setting new Velocity
            let self_new_vel = -collision_normal * velocity * other_energy / energy;
            let other_new_vel = collision_normal * velocity * self_energy / energy;

            self.velocity = self_new_vel;
            other.velocity = other_new_vel;

            // Seperating Circles
            let overlaping_depth = (other.position - self.position).length() - (self.radius + other.radius);
            self.position += collision_normal * overlaping_depth/2.;
            other.position -= collision_normal * overlaping_depth/2.;
        }

        pub fn collide_with(&mut self, other: &mut Circle) {
            let collision_normal = (other.position - self.position).normalize();
            // Seperating Circles
            let overlaping_depth = (other.position - self.position).length() - (self.radius + other.radius);
            self.position += collision_normal * overlaping_depth/2.;
            other.position -= collision_normal * overlaping_depth/2.;
        }

        pub fn is_touching_point(&self, point: Vec2) -> bool {
            (point - self.position).length() < self.radius
        }

        pub fn apply_force(&mut self, force: Vec2) {
            self.velocity += force;
        }

        pub fn position(&self) -> Vec2 {self.position}
        pub fn velocity(&self) -> Vec2 {self.velocity}
    }
}