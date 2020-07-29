use std::collections::VecDeque;

use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;

use super::input::InputState;

pub trait Scene {
    fn is_done(&self) -> bool;

    fn on_load(&mut self, canvas: &WindowCanvas) -> (Vec<String>, Vec<String>);
    fn on_late_load(&mut self, _canvas: &WindowCanvas, _textures: &[Texture], _fonts: &[Font]) {}
    fn on_unload(&mut self) {}

    fn poll_event(&mut self, _event: sdl2::event::Event) {}
    fn process_input(&mut self, input_state: &InputState);

    fn update(
        &mut self,
        delta_time: f32,
        scene_queue: &mut VecDeque<Box<dyn Scene>>,
        canvas: &WindowCanvas,
    );

    fn late_update(
        &mut self,
        _delta_time: f32,
        _scene_queue: &mut VecDeque<Box<dyn Scene>>,
        _canvas: &WindowCanvas,
    ) {
    }

    fn draw(
        &mut self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
        textures: &[Texture],
        fonts: &[Font],
    );
}
