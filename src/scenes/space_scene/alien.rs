pub const ALIEN_ROW_COUNT: u32 = 4;
pub const INITIAL_ALIEN_VELOCITY: f32 = 100.0;
pub const PER_LEVEL_ALIEN_VELOCITY_INCREASE: f32 = 20.0;
pub const ALIEN_VELOCITY_INCREMENT: f32 = 10.0;
pub const ALIEN_DROPDOWN_DISTANCE: f32 = 40.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlienDirection {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
pub struct Alien {
    pub x: f32,
    pub y: f32,

    pub is_hit: bool,
}

impl Alien {
    pub fn new(x: f32, y: f32) -> Alien {
        Alien {
            x,
            y,
            is_hit: false,
        }
    }
}

pub struct AlienData {
    pub width: u32,
    pub height: u32,

    pub velocity: f32,
    pub direction: AlienDirection,
    pub next_direction: Option<AlienDirection>,
    pub dropdown_distance: f32,

    pub has_hit_bottom: bool,

    pub texture_index: usize,
    pub death_sound: Option<sdl2::mixer::Chunk>,
}
