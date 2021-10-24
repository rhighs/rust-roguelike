use glam::Vec2 as gVec2;
use macroquad::prelude::next_frame;
use macroquad::color::{ BLACK, RED };
use macroquad::window::{ clear_background, screen_width, screen_height };
use macroquad::input::mouse_position;

/* init modules */
mod shapes;
mod motion;
mod world;
pub mod components;

fn mouse_pos() -> gVec2 {
    let mouse_pos = mouse_position();
    gVec2::new(mouse_pos.0, mouse_pos.1)
}

#[macroquad::main("BasicShapes")]
async fn main() {
    println!("{}", screen_height());
    let mut world = world::World::new();


    let mut square = shapes::Square::new(
        Some(&gVec2::new(10.0, 10.0)),
        Some(100.0),
        Some(100.0)
    );

    world.new_entity(
        Some(components::Health(30)),
        Some(components::Name("Cringe")),
        Some(components::Shape(&mut square)),
        Some(components::Physics::new(Some(gVec2::new(100.0, 100.0)),
        Some(gVec2::new(0.1, 0.0)),
        Some(gVec2::new(100.0, 100.0)), Some(0.1)))
    );

    let mut square1 = shapes::Square::new(
        Some(&gVec2::new(400.0, 10.0)),
        Some(100.0),
        Some(100.0)
    );

    world.new_entity(
        Some(components::Health(30)),
        Some(components::Name("Cringe")),
        Some(components::Shape(&mut square1)),
        Some(components::Physics::new(
            Some(gVec2::new(400.0, 100.0)),
            Some(gVec2::new(-0.1, 0.0)),
            Some(gVec2::new(100.0, 100.0)),
            Some(0.1)
            )
        )
    );

    loop {
        clear_background(BLACK);
        world.colliding_entities(0);
        world.render();
        world.update();
        next_frame().await
    }
}
