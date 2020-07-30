pub mod bullet;

use sdl2::mixer::Chunk;
use sdl2::rect::Rect;

use self::bullet::{Bullet, BulletData};

pub const SPACESHIP_VELOCITY: f32 = 500.0;
pub const SPACESHIP_SHOOT_DELAY: f32 = 0.3;

pub struct Spaceship {
    pub rect: Rect,

    pub x_velocity: f32,
    pub is_firing: bool,
    pub shoot_delay: f32,

    pub is_hit: bool,

    pub texture_index: usize,
    pub bullet_data: BulletData,
    pub bullets: Vec<Bullet>,

    pub shoot_sound: Option<Chunk>,
}
