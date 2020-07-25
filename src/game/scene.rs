use std::collections::VecDeque;

use sdl2::pixels::Color as Colour;

use super::input::InputState;

pub trait Scene {
    fn is_done(&self) -> bool;

    fn background_colour(&self) -> Colour {
        Colour::RGB(0, 0, 0)
    }

    fn on_load(&mut self) {}
    fn on_unload(&mut self) {}

    fn process_input(&mut self, input_state: &InputState);
    fn update(&mut self, delta_time: f32, scene_queue: &mut VecDeque<Box<dyn Scene>>);
    fn draw(&mut self, canvas: &mut sdl2::render::WindowCanvas);
}
