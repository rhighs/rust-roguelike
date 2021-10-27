use macroquad::input::{ KeyCode, get_last_key_pressed };

pub enum EventId {
    OnSpace,
    OnLeft,
    OnRight,
    OnUp,
    OnDown,
}

pub struct Events {
    on_space: Vec<fn()>,
    on_left: Vec<fn()>,
    on_right: Vec<fn()>,
    on_up: Vec<fn()>,
    on_down: Vec<fn()>
}

impl Events {
    pub fn on(&mut self, id: EventId, task: fn()) {
        match id {
            EventId::OnSpace    => self.on_space.push(task),
            EventId::OnLeft     => self.on_left.push(task),
            EventId::OnRight    => self.on_right.push(task),
            EventId::OnUp       => self.on_up.push(task),
            EventId::OnDown     => self.on_down.push(task),
            _                   => ()
        }
    }

    pub fn dispatch(&self, id: EventId) {
        let mut selected: Option<&Vec<fn()>> = None;
        match id {
            EventId::OnSpace    => selected = Some(&self.on_space),
            EventId::OnLeft     => selected = Some(&self.on_left),
            EventId::OnRight    => selected = Some(&self.on_right),
            EventId::OnUp       => selected = Some(&self.on_up),
            EventId::OnDown     => selected = Some(&self.on_down),
            _                   => ()
        }


        if selected != None {
            for sfn in selected.unwrap() {
                sfn();
            }
        }
    }

    pub fn handle(&self) {
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

pub static mut INPUT_EVENTS: Events = Events {
    on_space: Vec::new(),
    on_left: Vec::new(),
    on_right: Vec::new(),
    on_up: Vec::new(),
    on_down: Vec::new()
};
