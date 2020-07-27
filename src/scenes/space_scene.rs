use std::collections::VecDeque;
use std::fs;

use sdl2::{
    keyboard::Scancode,
    pixels::Color as Colour,
    render::{Texture, WindowCanvas},
};

use crate::game::input::InputState;
use crate::game::scene::Scene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(10, 10, 10);

const SPACESHIP_VELOCITY: f32 = 500.0;
const SPACESHIP_SHOOT_DELAY: f32 = 0.3;
const BULLET_VELOCITY: f32 = 650.0;

const ALIEN_ROW_COUNT: u32 = 4;
const ALIEN_VELOCITY: f32 = 100.0;
const ALIEN_DROPDOWN_DISTANCE: f32 = 25.0;

#[derive(Debug)]
struct Spaceship {
    x: f32,
    y: f32,
    width: u32,
    height: u32,

    x_velocity: f32,
    is_firing: bool,
    shoot_delay: f32,

    texture_index: usize,
    bullet_data: BulletData,
    bullets: Vec<Bullet>,
}

#[derive(Clone, Copy, Debug)]
struct BulletData {
    width: u32,
    height: u32,

    texture_index: usize,
}

#[derive(Debug)]
struct Bullet {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AlienDirection {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Alien {
    x: f32,
    y: f32,
}

impl Alien {
    fn new(x: f32, y: f32) -> Alien {
        Alien { x, y }
    }
}

struct AlienData {
    width: u32,
    height: u32,

    direction: AlienDirection,
    next_direction: Option<AlienDirection>,
    dropdown_distance: f32,

    texture_index: usize,
}

pub struct SpaceScene {
    has_window_focus: bool,
    is_done: bool,

    spaceship: Spaceship,
    alien_data: AlienData,
    aliens: Vec<Alien>,
}

impl SpaceScene {
    pub fn new() -> SpaceScene {
        SpaceScene {
            has_window_focus: true,
            is_done: false,
            spaceship: Spaceship {
                x: 0.0,
                y: 0.0,
                width: 0,
                height: 0,
                x_velocity: 0.0,
                is_firing: false,
                shoot_delay: 0.0,
                texture_index: 0,
                bullet_data: BulletData {
                    width: 0,
                    height: 0,
                    texture_index: 0,
                },
                bullets: vec![],
            },
            alien_data: AlienData {
                width: 0,
                height: 0,
                direction: AlienDirection::Right,
                next_direction: None,
                dropdown_distance: 0.0,
                texture_index: 0,
            },
            aliens: vec![],
        }
    }

    fn create_alien_fleet(&mut self, canvas: &WindowCanvas) {
        for alien_y in 0..ALIEN_ROW_COUNT {
            let mut alien_x = self.alien_data.width;

            while alien_x < canvas.viewport().width() - self.alien_data.width {
                self.aliens.push(Alien::new(
                    alien_x as f32,
                    self.alien_data.height as f32
                        + (self.alien_data.height as f32 * 1.5 * alien_y as f32),
                ));

                alien_x += self.alien_data.width * 2;
            }
        }
    }
}

impl Scene for SpaceScene {
    fn is_done(&self) -> bool {
        self.is_done
    }

    fn on_load(&mut self, canvas: &WindowCanvas) -> Vec<String> {
        let mut textures = vec![];

        for (current_index, texture_file) in fs::read_dir("assets/textures").unwrap().enumerate() {
            let texture_file = texture_file.unwrap();
            let texture_filepath = texture_file.path();
            let texture_filepath_string = texture_filepath
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();

            match texture_filepath_string.as_ref() {
                "ship.png" => {
                    self.spaceship.x = (canvas.viewport().width() / 2) as f32;
                    self.spaceship.texture_index = current_index;
                }
                "bullet.png" => self.spaceship.bullet_data.texture_index = current_index,
                "alien.png" => self.alien_data.texture_index = current_index,
                _ => (),
            }

            textures.push(texture_filepath.to_str().unwrap().to_owned());
        }

        textures
    }

    fn on_late_load(&mut self, canvas: &WindowCanvas, textures: &[Texture]) {
        let spaceship_texture_data = &textures[self.spaceship.texture_index].query();
        let bullet_texture_data = &textures[self.spaceship.bullet_data.texture_index].query();
        let alien_texture_data = &textures[self.alien_data.texture_index].query();

        self.spaceship.width = spaceship_texture_data.width;
        self.spaceship.height = spaceship_texture_data.height;
        self.spaceship.y = (canvas.viewport().height() as u32 - self.spaceship.height) as f32;

        self.spaceship.bullet_data.width = bullet_texture_data.width;
        self.spaceship.bullet_data.height = bullet_texture_data.height;

        self.alien_data.width = alien_texture_data.width;
        self.alien_data.height = alien_texture_data.height;

        self.create_alien_fleet(canvas);
    }

    fn on_unload(&mut self) {}

    fn poll_event(&mut self, event: sdl2::event::Event) {
        use sdl2::event::Event::*;
        use sdl2::event::WindowEvent::{Minimized as Minimised, *};

        if let Window {
            win_event: window_event,
            ..
        } = event
        {
            match window_event {
                FocusGained | Restored => self.has_window_focus = true,
                FocusLost | Minimised => self.has_window_focus = false,
                _ => (),
            }
        }
    }

    fn process_input(&mut self, input_state: &InputState) {
        if input_state.is_key_down(Scancode::Escape) {
            self.is_done = true;
        }

        self.spaceship.x_velocity = 0.0;

        if input_state.is_any_key_pressed(&[Scancode::A, Scancode::Left]) {
            self.spaceship.x_velocity -= 1.0;
        }

        if input_state.is_any_key_pressed(&[Scancode::D, Scancode::Right]) {
            self.spaceship.x_velocity += 1.0;
        }

        self.spaceship.is_firing = false;

        if input_state.is_key_pressed(Scancode::Space) && self.spaceship.shoot_delay <= 0.0 {
            self.spaceship.is_firing = true;
            self.spaceship.shoot_delay = SPACESHIP_SHOOT_DELAY;
        }
    }

    fn update(
        &mut self,
        delta_time: f32,
        _scene_queue: &mut VecDeque<Box<dyn Scene>>,
        canvas: &WindowCanvas,
    ) {
        if !self.has_window_focus {
            return;
        }

        self.spaceship.x += self.spaceship.x_velocity * delta_time * SPACESHIP_VELOCITY;
        self.spaceship.x = f32::min(
            f32::max(0.0, self.spaceship.x),
            canvas.viewport().width() as f32,
        );

        if self.spaceship.is_firing {
            self.spaceship.bullets.push(Bullet {
                x: self.spaceship.x,
                y: self.spaceship.y - (self.spaceship.height / 2) as f32,
            });
        }

        for bullet in &mut self.spaceship.bullets {
            bullet.y -= delta_time * BULLET_VELOCITY;
        }

        let bullet_delete_threshold = -2.0 * self.spaceship.bullet_data.height as f32;
        self.spaceship
            .bullets
            .retain(|bullet| bullet.y > bullet_delete_threshold);

        if self.spaceship.shoot_delay > 0.0 {
            self.spaceship.shoot_delay -= delta_time;
        }

        let mut switch_alien_direction = false;
        let movement = delta_time * ALIEN_VELOCITY;

        if self.alien_data.dropdown_distance > 0.0 {
            self.alien_data.dropdown_distance -= movement;
        }

        for alien in &mut self.aliens {
            match self.alien_data.direction {
                AlienDirection::Left => {
                    alien.x -= movement;

                    if alien.x <= self.alien_data.width as f32 / 2.0 {
                        switch_alien_direction = true;
                    }
                }
                AlienDirection::Right => {
                    alien.x += movement;

                    if alien.x >= (canvas.viewport().width() - self.alien_data.width / 2) as f32 {
                        switch_alien_direction = true;
                    }
                }
                AlienDirection::Down => {
                    alien.y += movement;

                    if self.alien_data.dropdown_distance <= 0.0 {
                        self.alien_data.direction = self.alien_data.next_direction.unwrap();
                        self.alien_data.next_direction = None;
                    }
                }
            }
        }

        if switch_alien_direction {
            self.alien_data.next_direction = match self.alien_data.direction {
                AlienDirection::Left => Some(AlienDirection::Right),
                AlienDirection::Right => Some(AlienDirection::Left),
                _ => unreachable!(),
            };

            self.alien_data.direction = AlienDirection::Down;
            self.alien_data.dropdown_distance = ALIEN_DROPDOWN_DISTANCE;
        }
    }

    fn draw(&mut self, canvas: &mut WindowCanvas, textures: &[sdl2::render::Texture]) {
        if !self.has_window_focus {
            return;
        }

        canvas.set_draw_color(BACKGROUND_COLOUR);
        canvas.clear();

        for bullet in &self.spaceship.bullets {
            let bullet_rect = sdl2::rect::Rect::from_center(
                sdl2::rect::Point::new(bullet.x as i32, bullet.y as i32),
                self.spaceship.bullet_data.width,
                self.spaceship.bullet_data.height,
            );

            canvas
                .copy(
                    &textures[self.spaceship.bullet_data.texture_index],
                    None,
                    bullet_rect,
                )
                .unwrap();
        }

        for alien in &self.aliens {
            let alien_rect = sdl2::rect::Rect::from_center(
                sdl2::rect::Point::new(alien.x as i32, alien.y as i32),
                self.alien_data.width,
                self.alien_data.height,
            );

            canvas
                .copy(&textures[self.alien_data.texture_index], None, alien_rect)
                .unwrap();
        }

        let spaceship_rect = sdl2::rect::Rect::from_center(
            sdl2::rect::Point::new(self.spaceship.x as i32, self.spaceship.y as i32),
            self.spaceship.width,
            self.spaceship.height,
        );

        canvas
            .copy(
                &textures[self.spaceship.texture_index],
                None,
                spaceship_rect,
            )
            .unwrap();
    }
}
