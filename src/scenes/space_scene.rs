mod alien;
mod spaceship;

use std::collections::VecDeque;
use std::fs;

use sdl2::{
    keyboard::Scancode,
    mouse::MouseButton,
    pixels::Color as Colour,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

use self::alien::*;
use self::spaceship::bullet::*;
use self::spaceship::*;
use crate::game::input::InputState;
use crate::game::scene::Scene;
use crate::scenes::main_menu_scene::MainMenuScene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(10, 10, 10);

const INITIAL_PLAYER_LIVES: u32 = 3;
const LEVEL_RESET_TIME: f32 = 1.0;

pub struct SpaceScene {
    has_window_focus: bool,
    is_done: bool,

    current_level: u32,
    player_lives: u32,
    level_reset_timeout: f32,

    spaceship: Spaceship,
    spaceship_size: (u32, u32),
    alien_data: AlienData,
    aliens: Vec<Alien>,
}

impl SpaceScene {
    pub fn new() -> SpaceScene {
        SpaceScene {
            has_window_focus: true,
            is_done: false,
            current_level: 1,
            player_lives: INITIAL_PLAYER_LIVES,
            level_reset_timeout: 0.0,
            spaceship: Spaceship {
                rect: Rect::new(0, 0, 0, 0),
                x_velocity: 0.0,
                is_firing: false,
                shoot_delay: 0.0,
                is_hit: false,
                texture_index: 0,
                bullet_data: BulletData {
                    width: 0,
                    height: 0,
                    texture_index: 0,
                },
                bullets: vec![],
            },
            spaceship_size: (0, 0),
            alien_data: AlienData {
                width: 0,
                height: 0,
                velocity: INITIAL_ALIEN_VELOCITY,
                direction: AlienDirection::Right,
                next_direction: None,
                dropdown_distance: 0.0,
                has_hit_bottom: false,
                texture_index: 0,
            },
            aliens: vec![],
        }
    }

    fn setup_objects(&mut self, canvas: &WindowCanvas) {
        self.spaceship.rect = Rect::from_center(
            Point::new(
                (canvas.viewport().width() / 2) as i32,
                (canvas.viewport().height() - self.spaceship_size.1) as i32,
            ),
            self.spaceship_size.0,
            self.spaceship_size.1,
        );

        self.spaceship.x_velocity = 0.0;
        self.spaceship.is_firing = false;
        self.spaceship.shoot_delay = 0.0;
        self.spaceship.is_hit = false;
        self.spaceship.bullets.clear();

        self.create_alien_fleet(canvas);
    }

    fn create_alien_fleet(&mut self, canvas: &WindowCanvas) {
        self.alien_data.velocity = INITIAL_ALIEN_VELOCITY
            + (self.current_level - 1) as f32 * PER_LEVEL_ALIEN_VELOCITY_INCREASE;
        self.alien_data.direction = AlienDirection::Right;
        self.alien_data.next_direction = None;
        self.alien_data.dropdown_distance = 0.0;
        self.alien_data.has_hit_bottom = false;

        self.aliens.clear();

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

    fn process_spaceship_input(&mut self, input_state: &InputState) {
        self.spaceship.x_velocity = 0.0;

        if input_state.is_any_key_pressed(&[Scancode::A, Scancode::Left]) {
            self.spaceship.x_velocity -= 1.0;
        }

        if input_state.is_any_key_pressed(&[Scancode::D, Scancode::Right]) {
            self.spaceship.x_velocity += 1.0;
        }

        self.spaceship.is_firing = false;

        if (input_state.is_key_pressed(Scancode::Space)
            || input_state.is_mouse_button_pressed(MouseButton::Left))
            && self.spaceship.shoot_delay <= 0.0
        {
            self.spaceship.is_firing = true;
            self.spaceship.shoot_delay = SPACESHIP_SHOOT_DELAY;
        }
    }

    fn update_spaceship(&mut self, delta_time: f32, canvas: &WindowCanvas) {
        self.spaceship.rect.set_x(
            (self.spaceship.rect.x() as f32
                + (self.spaceship.x_velocity * delta_time * SPACESHIP_VELOCITY)) as i32,
        );
        self.spaceship.rect.set_x(std::cmp::min(
            std::cmp::max(0, self.spaceship.rect.x()),
            (canvas.viewport().width() - self.spaceship.rect.width()) as i32,
        ));

        if self.spaceship.is_firing {
            self.spaceship.bullets.push(Bullet {
                x: (self.spaceship.rect.x() + self.spaceship.rect.width() as i32 / 2) as f32,
                y: (self.spaceship.rect.y()) as f32,
                has_hit_something: false,
            });
        }

        for bullet in &mut self.spaceship.bullets {
            bullet.y -= delta_time * BULLET_VELOCITY;
        }

        if self.spaceship.shoot_delay > 0.0 {
            self.spaceship.shoot_delay -= delta_time;
        }
    }

    fn update_aliens(&mut self, delta_time: f32, canvas: &WindowCanvas) {
        let mut switch_alien_direction = false;
        let movement = delta_time * self.alien_data.velocity;

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
                }
            }

            let alien_rect = sdl2::rect::Rect::from_center(
                sdl2::rect::Point::new(alien.x as i32, alien.y as i32),
                self.alien_data.width,
                self.alien_data.height,
            );

            for bullet in &mut self.spaceship.bullets {
                let bullet_rect = sdl2::rect::Rect::from_center(
                    sdl2::rect::Point::new(bullet.x as i32, bullet.y as i32),
                    self.spaceship.bullet_data.width,
                    self.spaceship.bullet_data.height,
                );

                if alien_rect.intersection(bullet_rect).is_some() {
                    alien.is_hit = true;
                    bullet.has_hit_something = true;
                }
            }

            if alien_rect.intersection(self.spaceship.rect).is_some() {
                alien.is_hit = true;
                self.spaceship.is_hit = true;
                self.level_reset_timeout = LEVEL_RESET_TIME;
            }

            if alien_rect.y() as u32 + alien_rect.height() >= canvas.viewport().height() {
                self.alien_data.has_hit_bottom = true;
                self.level_reset_timeout = LEVEL_RESET_TIME;
            }
        }

        if self.alien_data.direction == AlienDirection::Down {
            if self.alien_data.dropdown_distance > 0.0 {
                self.alien_data.dropdown_distance -= movement;
            } else {
                self.alien_data.direction = self.alien_data.next_direction.unwrap();
                self.alien_data.next_direction = None;
            }
        }

        if switch_alien_direction {
            self.alien_data.next_direction = match self.alien_data.direction {
                AlienDirection::Left => Some(AlienDirection::Right),
                AlienDirection::Right => Some(AlienDirection::Left),
                _ => unreachable!(),
            };

            self.alien_data.velocity += ALIEN_VELOCITY_INCREMENT;
            self.alien_data.direction = AlienDirection::Down;
            self.alien_data.dropdown_distance = ALIEN_DROPDOWN_DISTANCE;
        }
    }

    fn draw_bullets(&self, canvas: &mut WindowCanvas, bullet_texture: &Texture) {
        for bullet in &self.spaceship.bullets {
            let bullet_rect = sdl2::rect::Rect::from_center(
                sdl2::rect::Point::new(bullet.x as i32, bullet.y as i32),
                self.spaceship.bullet_data.width,
                self.spaceship.bullet_data.height,
            );

            canvas.copy(bullet_texture, None, bullet_rect).unwrap();
        }
    }

    fn draw_aliens(&self, canvas: &mut WindowCanvas, alien_texture: &Texture) {
        for alien in &self.aliens {
            let alien_rect = sdl2::rect::Rect::from_center(
                sdl2::rect::Point::new(alien.x as i32, alien.y as i32),
                self.alien_data.width,
                self.alien_data.height,
            );

            canvas.copy(alien_texture, None, alien_rect).unwrap();
        }
    }

    fn draw_spaceship(&self, canvas: &mut WindowCanvas, spaceship_texture: &Texture) {
        canvas
            .copy(spaceship_texture, None, self.spaceship.rect)
            .unwrap();
    }
}

impl Scene for SpaceScene {
    fn is_done(&self) -> bool {
        self.is_done
    }

    fn on_load(&mut self, _canvas: &WindowCanvas) -> Vec<String> {
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
                "ship.png" => self.spaceship.texture_index = current_index,
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

        self.spaceship_size = (spaceship_texture_data.width, spaceship_texture_data.height);

        self.spaceship.bullet_data.width = bullet_texture_data.width;
        self.spaceship.bullet_data.height = bullet_texture_data.height;

        self.alien_data.width = alien_texture_data.width;
        self.alien_data.height = alien_texture_data.height;

        self.setup_objects(canvas);
    }

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

        self.process_spaceship_input(input_state);
    }

    fn update(
        &mut self,
        delta_time: f32,
        scene_queue: &mut VecDeque<Box<dyn Scene>>,
        canvas: &WindowCanvas,
    ) {
        if !self.has_window_focus {
            return;
        }

        if self.is_done {
            scene_queue.push_back(Box::new(MainMenuScene::new()));
        }

        if self.level_reset_timeout <= 0.0 {
            self.update_spaceship(delta_time, canvas);
            self.update_aliens(delta_time, canvas);

            let bullet_delete_threshold = -2.0 * self.spaceship.bullet_data.height as f32;
            self.spaceship
                .bullets
                .retain(|bullet| bullet.y > bullet_delete_threshold && !bullet.has_hit_something);

            self.aliens.retain(|alien| !alien.is_hit);

            if self.aliens.is_empty() {
                self.level_reset_timeout = LEVEL_RESET_TIME;
            }
        } else {
            self.level_reset_timeout -= delta_time;
        }
    }

    fn late_update(
        &mut self,
        _delta_time: f32,
        scene_queue: &mut VecDeque<Box<dyn Scene>>,
        canvas: &WindowCanvas,
    ) {
        if self.level_reset_timeout <= 0.0 {
            if self.aliens.is_empty() {
                self.current_level += 1;
                self.setup_objects(canvas);
            } else if self.spaceship.is_hit || self.alien_data.has_hit_bottom {
                self.player_lives -= 1;

                if self.player_lives > 0 {
                    self.setup_objects(canvas);
                } else {
                    self.is_done = true;
                    scene_queue.push_back(Box::new(MainMenuScene::new()));
                }
            }
        }
    }

    fn draw(&mut self, canvas: &mut WindowCanvas, textures: &[sdl2::render::Texture]) {
        if !self.has_window_focus {
            return;
        }

        canvas.set_draw_color(BACKGROUND_COLOUR);
        canvas.clear();

        self.draw_bullets(canvas, &textures[self.spaceship.bullet_data.texture_index]);
        self.draw_aliens(canvas, &textures[self.alien_data.texture_index]);
        self.draw_spaceship(canvas, &textures[self.spaceship.texture_index]);
    }
}
