use std::collections::VecDeque;

use sdl2::pixels::Color as Colour;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::game::input::InputState;
use crate::game::scene::Scene;

pub struct SpaceScene {
    offset: f32,
    is_done: bool,
}

impl SpaceScene {
    pub fn new() -> SpaceScene {
        SpaceScene { offset: 0.0, is_done: false, }
    }
}

impl Scene for SpaceScene {
    fn is_done(&self) -> bool {
        self.is_done
    }

    fn background_colour(&self) -> Colour {
        Colour::RGB(230, 230, 230)
    }

    fn on_load(&mut self, texture_creator: &TextureCreator<WindowContext>) {}

    fn on_unload(&mut self) {}

    fn poll_event(&mut self, event: sdl2::event::Event) {}

    fn process_input(&mut self, input_state: &InputState) {
        if input_state.is_key_down(sdl2::keyboard::Scancode::Escape) {
            self.is_done = true;
        }
    }

    fn update(&mut self, delta_time: f32, _scene_queue: &mut VecDeque<Box<dyn Scene>>) {
        self.offset += delta_time * 20.0;
    }

    fn draw(&mut self, canvas: &mut sdl2::render::WindowCanvas) {
        canvas.set_draw_color(Colour::RGB(0, 255, 0));
        canvas
            .draw_line(
                sdl2::rect::Point::new(10 + self.offset as i32, 10 + self.offset as i32),
                sdl2::rect::Point::new(100 + self.offset as i32, 100 + self.offset as i32),
            )
            .unwrap();
    }
}
