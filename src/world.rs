use itertools::izip;
pub use crate::components::{ Health, Name, Shape, Physics };

pub struct World <'a> {
    health_components: Vec<Option<Health>>,
    name_components: Vec<Option<Name>>,
    shape_components: Vec<Option<Shape<'a>>>,
    physics_components: Vec<Option<Physics>>,
    n_entities: u32
}

impl<'a> World <'a> {
    pub fn new() -> Self {
        Self {
            health_components: Vec::new(),
            shape_components: Vec::new(),
            name_components: Vec::new(),
            physics_components: Vec::new(),
            n_entities: 0
        }
    }

    pub fn new_entity(&mut self, health: Option<Health>, name: Option<Name>, shape: Option<Shape<'a>>, physics: Option<Physics>) -> u32 {
        self.n_entities += 1;
        self.health_components.push(health);
        self.name_components.push(name);
        self.shape_components.push(shape);
        self.physics_components.push(physics);
        self.n_entities
    }

    fn erase_entity(&mut self, idx: usize) -> bool {
        let erased: bool =
            self.health_components[idx].is_some()
            && self.name_components[idx].is_some()
            && self.shape_components[idx].is_some();

        self.health_components[idx] = None;
        self.name_components[idx] = None;
        self.shape_components[idx] = None;

        erased
    }

    pub fn update(&mut self) {
        let len = self.physics_components.len();

        for i in 0 .. len {
            let physics_ref = self.physics_components[i].as_mut().unwrap();
            physics_ref.update();
            let shape_ref = self.shape_components[i].as_mut().unwrap();
            shape_ref.0.set_pos(&physics_ref.position);
        }
    }

    pub fn colliding_entities(&mut self, id: usize) -> Vec<Physics> {
        let len = self.physics_components.len();
        let mut data = self.physics_components.clone();
        let subject = self.physics_components[id].as_mut().unwrap();
        let mut collides: Vec<Physics> = Vec::new();

        for i in 0..len {
            if i == id {
                continue;
            }
            let target = data[i].as_mut().unwrap();

            if subject.check_collision(&target) {
                collides.push(*target);
                self.physics_components[i].as_mut().unwrap().stop_mov();
                subject.stop_mov();
            }
        }

        collides
    }

    pub fn render(&self) {
        for shape in &self.shape_components {
            shape.as_ref().unwrap().0.draw();
        }
    }
}
