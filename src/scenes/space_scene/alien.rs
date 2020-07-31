use rand::Rng;
use sdl2::mixer::Chunk;

use super::spaceship::bullet::{Bullet, BulletData};

pub const ALIEN_ROW_COUNT: u32 = 4;
pub const INITIAL_ALIEN_VELOCITY: f32 = 100.0;
pub const PER_LEVEL_ALIEN_VELOCITY_INCREASE: f32 = 20.0;
pub const ALIEN_VELOCITY_INCREMENT: f32 = 10.0;
pub const ALIEN_DROPDOWN_DISTANCE: f32 = 40.0;
pub const ALIEN_SHOOT_INTERVAL: f32 = 10.0;
pub const ALIEN_BASE_POINTS: u32 = 25;

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

    pub shoot_delay: f32,
    pub is_hit: bool,
}

impl Alien {
    pub fn new(x: f32, y: f32) -> Alien {
        Alien {
            x,
            y,
            shoot_delay: rand::thread_rng().gen::<f32>() * ALIEN_SHOOT_INTERVAL,
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
    pub bullet_data: BulletData,
    pub bullets: Vec<Bullet>,

    pub shoot_sound: Option<Chunk>,
    pub death_sound: Option<Chunk>,
    pub pass_sound: Option<Chunk>,
    pub shift_sound: Option<Chunk>,
}
