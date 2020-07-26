use std::collections::VecDeque;

use super::input::InputState;

pub trait Scene {
    fn is_done(&self) -> bool;

    fn on_load(&mut self) -> Vec<String>;
    fn on_unload(&mut self) {}

    fn poll_event(&mut self, event: sdl2::event::Event);
    fn process_input(&mut self, input_state: &InputState);
    fn update(&mut self, delta_time: f32, scene_queue: &mut VecDeque<Box<dyn Scene>>);
    fn draw(&mut self, canvas: &mut sdl2::render::WindowCanvas, textures: &[sdl2::render::Texture]);
}
