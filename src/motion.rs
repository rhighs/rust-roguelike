use glam::{ Vec2 };
use std::f64::consts::{ PI };

pub struct Circular {
    vel: f32,
    angle: f32,
    radius: f32,
    origin: Vec2
}

impl Circular { 
    pub fn new(radius: f32, vel: f32, angle: f32, origin: &Vec2) -> Circular {
        Circular {
            vel,
            angle,
            radius,
            origin: origin.clone()
        }
    }

    pub fn set_origin(&mut self, pos: &Vec2) {
        self.origin = pos.clone();
    }

    pub fn next_pos(&mut self) -> Vec2 {
        if self.angle >= (2.0 * PI) as f32 {
            self.angle = 0.0;
        }

        let pos = Vec2::new(
            self.angle.cos() * self.radius + self.origin.x,
            self.angle.sin() * self.radius + self.origin.y
            );

        self.angle += self.vel;
        pos
    }
}

