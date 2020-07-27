use std::collections::VecDeque;
use std::fs;

use sdl2::{
    keyboard::Scancode,
    pixels::Color as Colour,
    render::{Texture, WindowCanvas},
};

use crate::game::input::InputState;
use crate::game::scene::Scene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(25, 25, 25);
const SPACESHIP_VELOCITY: f32 = 500.0;

#[derive(Debug)]
struct Spaceship {
    x: f32,
    y: f32,
    width: u32,
    height: u32,

    x_velocity: f32,

    texture_index: usize,
}

pub struct SpaceScene {
    has_window_focus: bool,
    is_done: bool,

    spaceship: Spaceship,
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
                texture_index: 0,
            },
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
                    self.spaceship = Spaceship {
                        x: (canvas.viewport().width() / 2) as f32,
                        y: 0.0,
                        width: 0,
                        height: 0,
                        x_velocity: 0.0,
                        texture_index: current_index,
                    };
                }
                "bullet.png" => (),
                _ => (),
            }

            textures.push(texture_filepath.to_str().unwrap().to_owned());
        }

        textures
    }

    fn on_late_load(&mut self, canvas: &WindowCanvas, textures: &[Texture]) {
        let spaceship_texture_data = &textures[self.spaceship.texture_index].query();

        self.spaceship.width = spaceship_texture_data.width;
        self.spaceship.height = spaceship_texture_data.height;
        self.spaceship.y = (canvas.viewport().height() as u32 - self.spaceship.height) as f32;
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
    }

    fn draw(&mut self, canvas: &mut WindowCanvas, textures: &[sdl2::render::Texture]) {
        if !self.has_window_focus {
            return;
        }

        canvas.set_draw_color(BACKGROUND_COLOUR);
        canvas.clear();

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
