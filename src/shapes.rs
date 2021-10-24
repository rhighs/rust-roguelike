use macroquad::shapes::*;
use macroquad::color::{ Color };

use glam::Vec2;
use std::any::Any;

pub trait Shape {
    fn draw(&self);
    fn as_any(&self) -> &dyn Any;
    fn set_pos(&mut self, pos: &Vec2);
}

#[derive(Clone, Copy)]
pub struct Square {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color
}
impl Shape for Square{
    fn draw(&self) {
        draw_rectangle(
            self.x,
            self.y,
            self.w,
            self.h,
            self.color
            );
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_pos(&mut self, pos: &Vec2) {
        self.x = pos.x;
        self.y = pos.y;
    }
}

#[derive(Clone, Copy)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
    pub color: Color
}

impl Shape for Circle{
    fn draw(&self) {
        draw_circle(self.center.x, self.center.y, self.radius, self.color)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_pos(&mut self, pos: &Vec2) {
        self.center = pos.clone();
    }
}

impl Square {
    pub fn new(pos: Option<&Vec2>, w: Option<f32>, h: Option<f32>) -> Square {
        let vec = pos.unwrap();
        Square {
            x: vec.x,
            y: vec.y,
            w: w.unwrap(),
            h: h.unwrap(),
            color: Color::new(0.0, 200.0, 0.0, 1.0)
        }
    }
}

impl Circle {
    pub fn new(center: &Vec2, radius: f32, color: &Color) -> Circle {
        Circle {
            center: center.clone(),
            radius: radius, 
            color: color.clone()
        }
    }
}
