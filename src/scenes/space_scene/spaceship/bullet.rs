pub const BULLET_VELOCITY: f32 = 650.0;

#[derive(Debug)]
pub struct BulletData {
    pub width: u32,
    pub height: u32,

    pub texture_index: usize,
}

#[derive(Debug)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,

    pub has_hit_something: bool,
}
