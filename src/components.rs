use crate::shapes::Shape as Shape_t;

use glam::Vec2;

pub struct Health(pub i32);
pub struct Name(pub &'static str);
pub struct Shape<'a>(pub &'a mut dyn Shape_t);

#[derive(Clone, Copy)]
pub struct Physics {
    pub position: Vec2,
    velocity: Vec2,
    bounding: Vec2,
    mass: f32
}

impl Physics {
    pub fn new(position: Option<Vec2>, velocity: Option<Vec2>, bounding: Option<Vec2>, mass: Option<f32>) -> Self {
        Physics {
            position: position.unwrap_or(Vec2::new(0.0, 0.0)),
            velocity: velocity.unwrap_or(Vec2::new(0.0, 0.0)),
            bounding: bounding.unwrap_or(Vec2::new(0.0, 0.0)),
            mass: mass.unwrap_or(1.0),
        }
    }

    pub fn set_pos(&mut self, position: &Vec2) {
        self.position.x = position.x;
        self.position.y = position.y;
    }

    pub fn stop_mov(&mut self) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
    }

    //proudly stolen from:
    //https://math.stackexchange.com/questions/7356/how-to-find-rectangle-intersection-on-a-coordinate-plane
    pub fn check_collision(&self, physics_obj: &Physics) -> bool {
        let leftx = f32::max(physics_obj.position.x, self.position.x);
        let rightx = f32::min(physics_obj.position.x + physics_obj.bounding.x,
                              self.position.x + self.bounding.x);
        let topy = f32::max(physics_obj.position.y, self.position.y);
        let bottomy = f32::min(physics_obj.position.y + physics_obj.bounding.y,
                              self.position.y + self.bounding.y);
        leftx < rightx && topy < bottomy
    }

    pub fn step(&mut self) {
        self.position += self.velocity;
    }

    pub fn update(&mut self) {
        self.step();
    }
}
