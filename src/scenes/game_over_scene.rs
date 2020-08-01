use std::collections::VecDeque;
use std::fs;

use sdl2::keyboard::Scancode;
use sdl2::pixels::Color as Colour;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;

use crate::game::input::InputState;
use crate::game::scene::Scene;
use crate::scenes::main_menu_scene::MainMenuScene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(10, 10, 10);

pub struct GameOverScene {
	player_score: u32,

	is_done: bool,
	font_index: usize,
}

impl GameOverScene {
	pub fn new() -> GameOverScene {
		GameOverScene {
			player_score: 0,
			is_done: false,
			font_index: 0,
		}
	}
}

impl Scene for GameOverScene {
	fn is_done(&self) -> bool {
		self.is_done
	}

    fn on_load(
        &mut self,
        _canvas: &WindowCanvas,
        previous_scene_payload: Option<i32>,
    ) -> (Vec<String>, Vec<String>) {
		self.player_score = previous_scene_payload.unwrap_or(0) as u32;
		let mut fonts = vec![];

		for (current_index, font_file) in fs::read_dir("assets/fonts").unwrap().enumerate() {
            let font_file = font_file.unwrap();
            let font_filepath = font_file.path();
            let font_filepath_string = font_filepath
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();

            if font_filepath_string == "Recursive.ttf" {
                self.font_index = current_index;
            }

            fonts.push(font_filepath.to_str().unwrap().to_owned());
		}
		
		(vec![], fonts)
	}

	fn on_late_load(&mut self, _canvas: &WindowCanvas, _textures: &[Texture], _fonts: &[Font]) {}
	
    fn process_input(&mut self, input_state: &InputState) {
		if input_state.is_key_down(Scancode::Escape) {
			self.is_done = true;
		}
	}

    fn update(
        &mut self,
        _delta_time: f32,
        scene_queue: &mut VecDeque<Box<dyn Scene>>,
        canvas: &WindowCanvas,
        sound_channel: &sdl2::mixer::Channel,
    ) {
		if self.is_done {
			scene_queue.push_back(Box::new(MainMenuScene::new()));

			return;
		}
	}

    fn draw(
        &mut self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
        _textures: &[Texture],
        fonts: &[Font],
    ) {
		canvas.set_draw_color(BACKGROUND_COLOUR);
        canvas.clear();
	}
}