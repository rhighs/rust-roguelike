use macroquad::input::{ KeyCode, get_last_key_pressed };

pub enum EventId {
    OnSpace,
    OnLeft,
    OnRight,
    OnUp,
    OnDown,
}

pub struct Events<'a> {
    on_space: Vec<&'a dyn FnMut() -> ()>,
    on_left: Vec<&'a dyn FnMut() -> ()>,
    on_right: Vec<&'a dyn FnMut() -> ()>,
    on_up: Vec<&'a dyn FnMut() -> ()>,
    on_down: Vec<&'a dyn FnMut() -> ()>
}

impl<'a> Events <'a> {
    pub fn new() -> Self {
        Self {
            on_space: Vec::new(),
            on_left: Vec::new(),
            on_right: Vec::new(),
            on_up: Vec::new(),
            on_down: Vec::new()
        }
    }

    pub fn on(&mut self, id: EventId, task: &'a dyn FnMut() -> ()) {
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
        let mut selected: Option<&Vec<& dyn FnMut() -> ()> = None;

        match id {
            EventId::OnSpace    => selected = Some(&self.on_space),
            EventId::OnLeft     => selected = Some(&self.on_left),
            EventId::OnRight    => selected = Some(&self.on_right),
            EventId::OnUp       => selected = Some(&self.on_up),
            EventId::OnDown     => selected = Some(&self.on_down),
            _                   => ()
        }

        if !selected.is_none() {
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

