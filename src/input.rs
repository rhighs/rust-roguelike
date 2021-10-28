use macroquad::input::{ KeyCode, get_last_key_pressed };

pub enum EventId {
    OnSpace,
    OnLeft,
    OnRight,
    OnUp,
    OnDown,
}

pub struct Events {
    on_space: Vec<Box<dyn FnMut() -> ()>>,
    on_left: Vec<Box<dyn FnMut() -> ()>>,
    on_right: Vec<Box<dyn FnMut() -> ()>>,
    on_up: Vec<Box<dyn FnMut() -> ()>>,
    on_down: Vec<Box<dyn FnMut() -> ()>>
}

impl Events {
    pub fn new() -> Self {
        Self {
            on_space: Vec::new(),
            on_left: Vec::new(),
            on_right: Vec::new(),
            on_up: Vec::new(),
            on_down: Vec::new()
        }
    }

    pub fn on(&mut self, id: EventId, task: Box<dyn FnMut() -> ()>) {
        match id {
            EventId::OnSpace    => self.on_space.push(task),
            EventId::OnLeft     => self.on_left.push(task),
            EventId::OnRight    => self.on_right.push(task),
            EventId::OnUp       => self.on_up.push(task),
            EventId::OnDown     => self.on_down.push(task),
            _                   => ()
        }
    }

    pub fn dispatch(&mut self, id: EventId) {
        let selected = match id {
            EventId::OnSpace    => &mut self.on_space,
            EventId::OnLeft     => &mut self.on_left,
            EventId::OnRight    => &mut self.on_right,
            EventId::OnUp       => &mut self.on_up,
            EventId::OnDown     => &mut self.on_down
        };

        for sfn in selected {
            sfn();
        }
    }

    pub fn handle(&mut self) {
        let pressed = get_last_key_pressed()
            .unwrap_or(KeyCode::Unknown);

        match pressed {
            KeyCode::Space  => self.dispatch(EventId::OnSpace),
            KeyCode::Down   => self.dispatch(EventId::OnLeft),
            KeyCode::Up     => self.dispatch(EventId::OnRight),
            KeyCode::Left   => self.dispatch(EventId::OnLeft),
            KeyCode::Right  => self.dispatch(EventId::OnRight),
            _               => ()
        }
    }
}

