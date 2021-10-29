pub use crate::components::{ Health, Name, Shape, Physics };
pub use crate::input::{ Events, EventId };

pub struct World<'a>{
    health_components: Vec<Option<Health>>,
    name_components: Vec<Option<Name>>,
    shape_components: Vec<Option<Shape<'a>>>,
    physics_components: Vec<Option<Physics>>,
    input_handle: Events<'a>,
    n_entities: usize
}

impl<'a> World<'a>{
    pub fn new() -> Self {
        Self {
            health_components: Vec::new(),
            shape_components: Vec::new(),
            name_components: Vec::new(),
            physics_components: Vec::new(),
            input_handle: Events::new(),
            n_entities: 0
        }
    }

    pub fn new_entity(&mut self, health: Option<Health>,
                      name: Option<Name>, shape: Option<Shape<'a>>,
                      physics: Option<Physics>) -> usize {
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

    pub fn check_world_collisions(&mut self) {
        let len = self.physics_components.len();
        for i in 0..len {
            self.colliding_entities(i);
        }
    }

    pub fn add_player(&mut self, health: Option<Health>,
                      name: Option<Name>, shape: Option<Shape<'a>>,
                      physics: Option<Physics>) {
        self.new_entity(health, name, shape, physics);

        let os: Box<dyn FnMut() -> () + 'static> = Box::new(move || {
            physics.unwrap().step();
        });

        self.input_handle.on(EventId::OnSpace,  Box::new(os));
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
                subject.stop_mov();
            }
        }

        collides
    }

    pub fn update(&mut self) {
        let len = self.physics_components.len();

        self.input_handle.handle();

        //update shape screen pos according to pos given by the physics component
        for i in 0 .. len {
            let physics_ref = self.physics_components[i].as_mut().unwrap();
            physics_ref.update();
            self.shape_components[i].as_mut().unwrap().0.set_pos(&physics_ref.position);
        }
    }

    pub fn render(&self) {
        for shape in &self.shape_components {
            shape.as_ref().unwrap().0.draw();
        }
    }
}
