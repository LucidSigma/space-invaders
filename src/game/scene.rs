use std::collections::VecDeque;

use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;

use super::input::InputState;

pub trait Scene {
    fn is_done(&self) -> bool;

    fn on_load(
        &mut self,
        sdl_context: &sdl2::Sdl,
        canvas: &WindowCanvas,
        previous_scene_payload: Option<i32>,
    ) -> (Vec<String>, Vec<String>);
    fn on_late_load(&mut self, _canvas: &WindowCanvas, _textures: &[Texture], _fonts: &[Font]) {}

    fn on_unload(&mut self, _sdl_context: &sdl2::Sdl) -> Option<i32> {
        None
    }

    fn poll_event(&mut self, _event: sdl2::event::Event) {}
    fn process_input(&mut self, input_state: &InputState);

    fn update(
        &mut self,
        delta_time: f32,
        scene_queue: &mut VecDeque<Box<dyn Scene>>,
        canvas: &WindowCanvas,
        sound_channel: &sdl2::mixer::Channel,
    );

    fn late_update(
        &mut self,
        _delta_time: f32,
        _scene_queue: &mut VecDeque<Box<dyn Scene>>,
        _canvas: &WindowCanvas,
        _sound_channel: &sdl2::mixer::Channel,
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
