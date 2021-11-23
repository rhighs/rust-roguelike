use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;
use tcod::colors;
use tcod::map::{Map as FovMap};

use rand::Rng;

mod consts;
use consts::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum DeathCallback {
    Player,
    Monster
}

impl DeathCallback {
    fn callback(self, object: &mut Object, game: &mut Game) {
        use DeathCallback::*;
        let callback = match self {
            Player => player_death,
            Monster => monster_death
        };
        callback(object, game);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Fighter {
    max_hp: i32,
    hp: i32,
    defense: i32,
    power: i32,
    on_death: DeathCallback
}

#[derive(Clone, Debug, PartialEq)]
enum Ai {
    Basic
}

struct Tcod {
    root: Root,
    con: Offscreen,
    panel: Offscreen,
    fov: FovMap
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
    explored: bool
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            blocked: false,
            explored: false,  
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Self {
            blocked: true,
            explored: false,  
            block_sight: true,
        }
    }
}

struct Messages {
    messages: Vec<(String, Color)>
}

impl Messages {
    pub fn new() -> Self {
        Self {
            messages: vec![]
        }
    }

    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color));
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
        self.messages.iter()
    }
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
    messages: Messages
}

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32, 
    char: char,
    color: Color,
    name: String,
    blocks: bool,
    alive: bool,
    fighter: Option<Fighter>,
    ai: Option<Ai>
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Self {
            x,
            y,
            char, 
            color,
            name: name.into(),
            blocks, 
            alive: false,
            fighter: None,
            ai: None
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn take_damage(&mut self, damage: i32, game: &mut Game) {
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }

        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self, game);
            }
        }
    }

    pub fn attack(&mut self, target: &mut Object, game: &mut Game) {
        let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);
        if damage > 0 {
            game.messages.add(
                format!("{} attacks {} for {} hitpoints", self.name, target.name, damage),
                GREEN
            );
            target.take_damage(damage, game);
        } else {
            game.messages.add(
                format!("{} attacks {} but it has no effect", self.name, target.name),
                WHITE
            );
        }
    }
}

fn player_death(player: &mut Object, game: &mut Game) {
    game.messages.add(
        "You died!",
        RED
    );
    player.char = '%';
    player.color = DARK_RED;
}

fn monster_death(monster: &mut Object, game: &mut Game) {
    game.messages.add(
        format!("{} is dead!", monster.name),
        RED
    );
    monster.char = '%';
    monster.color = DARK_RED;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("Remains of {}", monster.name);
}

fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[id].pos();
    if !is_blocked(x + dx, y + dy, map, objects) {
        objects[id].set_pos(x + dx, y + dy);
    }
}

fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
    let dx = (dx as f32 / distance) as i32;
    let dy = (dy as f32 / distance) as i32;
    move_by(id, dx, dy, map, objects);
}

fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
    let (monster_x, monster_y) = objects[monster_id].pos();
    if objects[monster_id].distance_to(&objects[PLAYER_IDX]) >= 2.0 {
        let (player_x, player_y) = objects[PLAYER_IDX].pos();
        move_towards(monster_id, player_x, player_y, &game.map, objects);
    } else if objects[PLAYER_IDX].fighter.map_or(false, |f| f.hp > 0) {
        let (monster, player) = mut_two(monster_id, PLAYER_IDX, objects);
        monster.attack(player, game);
    }
}

fn mut_two<T>(first_idx: usize, second_idx: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_idx != second_idx);
    let split_at_idx = std::cmp::max(first_idx, second_idx);
    let (first_slice, second_slice) = items.split_at_mut(split_at_idx);
    if first_idx < second_idx {
        //splitted at second idx, so second_idx will be -> 0 as idx
        (&mut first_slice[first_idx], &mut second_slice[0])
    } else {
        //splitted at fitst idx, so first_idx will be -> 0 as idx
        (&mut second_slice[0], &mut first_slice[second_idx])
    }
}

fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
    let x = objects[PLAYER_IDX].x + dx;
    let y = objects[PLAYER_IDX].y + dy;

    let target_id = objects
        .iter()
        .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
            let (monster, player) = mut_two(target_id, PLAYER_IDX, objects);
            player.attack(monster, game);
        }
        None => {
            move_by(PLAYER_IDX, dx, dy, &game.map, objects);
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    for x in std::cmp::min(x1, x2)..(std::cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // vertical tunnel
    for y in std::cmp::min(y1, y2)..(std::cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_room(room: Rect, map: &mut Map) {
    // +1 to be contained inside walls (wall_width is 1)
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn make_map(objects: &mut Vec<Object>) -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let mut rooms = vec![];

    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE..(ROOM_MAX_SIZE + 1));
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE..(ROOM_MAX_SIZE + 1));
        let x = rand::thread_rng().gen_range(0..(MAP_WIDTH - w));
        let y = rand::thread_rng().gen_range(0..(MAP_HEIGHT - h));

        let room: Rect = Rect::new(x, y, w, h);
        let failed = rooms
            .iter()
            .any(|other| room.intersects_with(other));

        let (new_x, new_y) = room.center();

        if !failed {
            create_room(room, &mut map);
            place_objects(room, &map, objects);
            if rooms.is_empty() {
                let player = &mut objects[PLAYER_IDX];
                player.set_pos(new_x, new_y)
            } else {

                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
        }

        rooms.push(room);
    }

    map
}

fn render_all(tcod: &mut Tcod, game: &mut Game, objects: &[Object], fov_recompute: bool) {
    if fov_recompute {
        // recompute FOV if needed (the player moved or something)
        let player = &objects[PLAYER_IDX];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match (visible, wall) {
                // outside of field of view:
                (false, true) => COLOR_DARK_WALL,
                (false, false) => COLOR_DARK_GROUND,
                // inside fov:
                (true, true) => COLOR_LIGHT_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
            };

            let explored = &mut game.map[x as usize][y as usize].explored;
            if visible {
                // since it's visible, explore it
                *explored = true;
            }
            if *explored {
                // show explored tiles only (any visible tile is explored already)
                tcod.con
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    let mut to_draw: Vec<_> = objects
        .iter()
        .filter(|o| tcod.fov.is_in_fov(o.x, o.y))
        .collect();
    to_draw.sort_by(|o1, o2| { o1.blocks.cmp(&o2.blocks) });

    for object in &to_draw {
        object.draw(&mut tcod.con);
    }

    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
        );

    tcod.panel.set_default_background(BLACK);
    tcod.panel.clear();

    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    let hp = objects[PLAYER_IDX].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER_IDX].fighter.map_or(0, |f| f.max_hp);

    render_bar(
        &mut tcod.panel,
        1,
        1,
        BAR_WIDTH,
        "hp",
        hp,
        max_hp,
        LIGHT_RED,
        DARKER_RED
    );

    blit(
        &tcod.panel,
        (0, 0),
        (SCREEN_WIDTH, PANEL_HEIGHT),
        &mut tcod.root,
        (0, PANEL_Y),
        1.0,
        1.0,
    );
}

fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> PlayerAction {
    let key = tcod.root.wait_for_keypress(true);
    let player_alive = objects[PLAYER_IDX].alive;
    match (key, key.text(), player_alive){
        (
            Key {
            code: Enter,
            alt: true,
            ..
            },
            _,
            _
        ) => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            PlayerAction::DidntTakeTurn
        },
        (Key { code: Escape, .. }, _, _) => return PlayerAction::Exit, // exit game
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, game, objects);
            PlayerAction::TookTurn
        },
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, game, objects);
            PlayerAction::TookTurn
        },
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, game, objects);
            PlayerAction::TookTurn
        },
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, game, objects);
            PlayerAction::TookTurn
        },

        _ => PlayerAction::DidntTakeTurn
    }
}

fn place_objects(room: Rect, map: &Map, objects: &mut Vec<Object>) {
    let num_monsters = rand::thread_rng().gen_range(0..MAX_ROOM_MONSTERS + 1);
    for _ in 0..num_monsters {
        let x = rand::thread_rng().gen_range((room.x1 + 1)..room.x2);
        let y = rand::thread_rng().gen_range((room.y1 + 1)..room.y2);

        if !is_blocked(x, y, map, objects) {
            let mut monster = if rand::random::<f32>() < 0.8 {
                let mut orc = Object::new(x, y, 'o', colors::DESATURATED_GREEN, "Orc", true);
                orc.fighter = Some(Fighter {
                    max_hp: 10,
                    hp: 10,
                    defense: 0,
                    power: 3,
                    on_death: DeathCallback::Monster
                });
                orc.ai = Some(Ai::Basic);
                orc
            } else {
                let mut troll = Object::new(x, y, 'T', colors::DARKER_GREEN, "Troll", true);
                troll.fighter = Some(Fighter {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 4,
                    on_death: DeathCallback::Monster
                });
                troll.ai = Some(Ai::Basic);
                troll
            };
            monster.alive = true;
            objects.push(monster);
        }
    }
}

fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    if map[x as usize][y as usize].blocked {
        return true;
    }

    objects
        .iter()
        .any(|object| object.blocks && object.pos() == (x, y))
}

fn render_bar(
    panel: &mut Offscreen,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    max: i32,
    bar_color: Color,
    background_color: Color
    ) {
    let bar_width = ((value as f32 / max as f32) * (total_width as f32)) as i32;
    panel.set_default_background(background_color);
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);
    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }
    panel.set_default_foreground(WHITE);
    panel.print_ex(
        x + total_width / 2,
        y,
        BackgroundFlag::None,
        TextAlignment::Center,
        &format!("{}: {}/{}", name, value, max)
    )
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let panel = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let root: Root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let mut tcod = Tcod {
        root,
        con,
        panel,
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT)
    };

    let mut player = Object::new(25, 23, '@', WHITE, "Skumonti", true);
    player.alive = true;
    player.fighter = Some(Fighter {
        max_hp: 30,
        hp: 30,
        defense: 2,
        power: 5,
        on_death: DeathCallback::Player
    });
    let mut objects = Vec::from([player]);

    let mut game = Game {
        map: make_map(&mut objects),
        messages: Messages::new()
    };

    game.messages.add(
        "Welcome stranger! Prepare to perish in the Tombs of skumonti",
        RED
    );

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x,
                y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
                );
        }
    }

    let mut previous_player_position = (-1, -1);

    while !tcod.root.window_closed() {
        tcod.con.clear();

        let fov_recompute = previous_player_position != objects[PLAYER_IDX].pos();
        render_all(&mut tcod, &mut game, &objects, fov_recompute);

        tcod.root.flush();

        previous_player_position = objects[PLAYER_IDX].pos();
        let player_action = handle_keys(&mut tcod, &mut game, &mut objects);
        if player_action == PlayerAction::Exit {
            break;
        }

        if objects[PLAYER_IDX].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &tcod, &mut game, &mut objects);
                }
            }
        }
    }
}

