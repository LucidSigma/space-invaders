use std::collections::VecDeque;
use std::fs;

use sdl2::pixels::Color as Colour;

use crate::game::input::InputState;
use crate::game::scene::Scene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(230, 230, 230);

pub struct SpaceScene {
    offset: f32,
    has_window_focus: bool,
    is_done: bool,
    spaceship_texture_index: usize,
}

impl SpaceScene {
    pub fn new() -> SpaceScene {
        SpaceScene {
            offset: 0.0,
            has_window_focus: true,
            is_done: false,
            spaceship_texture_index: 0,
        }
    }
}

impl Scene for SpaceScene {
    fn is_done(&self) -> bool {
        self.is_done
    }

    fn on_load(&mut self) -> Vec<String> {
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
                "ship.png" => self.spaceship_texture_index = current_index,
                _ => (),
            }

            textures.push(texture_filepath.to_str().unwrap().to_owned());
        }

        textures
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
        if input_state.is_key_down(sdl2::keyboard::Scancode::Escape) {
            self.is_done = true;
        }
    }

    fn update(&mut self, delta_time: f32, _scene_queue: &mut VecDeque<Box<dyn Scene>>) {
        if !self.has_window_focus {
            return;
        }

        self.offset += delta_time * 40.0;
    }

    fn draw(
        &mut self,
        canvas: &mut sdl2::render::WindowCanvas,
        textures: &[sdl2::render::Texture],
    ) {
        if !self.has_window_focus {
            return;
        }

        canvas.set_draw_color(BACKGROUND_COLOUR);
        canvas.clear();

        canvas
            .copy(
                &textures[self.spaceship_texture_index],
                sdl2::rect::Rect::new(0, 0, 60, 48),
                sdl2::rect::Rect::new(0, 0, 1200, 800),
            )
            .unwrap();

        canvas.set_draw_color(Colour::RGB(0, 0, 0));
        canvas
            .draw_line(
                sdl2::rect::Point::new(10 + self.offset as i32, 10 + self.offset as i32),
                sdl2::rect::Point::new(100 + self.offset as i32, 100 + self.offset as i32),
            )
            .unwrap();
    }
}
