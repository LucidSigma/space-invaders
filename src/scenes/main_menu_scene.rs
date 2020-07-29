use std::collections::VecDeque;
use std::fs;

use sdl2::keyboard::Scancode;
use sdl2::pixels::Color as Colour;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;

use crate::game::input::InputState;
use crate::game::scene::Scene;
use crate::scenes::space_scene::SpaceScene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(10, 10, 10);

pub struct MainMenuScene {
    font_index: usize,

    load_game_scene: bool,
    is_done: bool,
}

impl MainMenuScene {
    pub fn new() -> MainMenuScene {
        MainMenuScene {
            font_index: 0,
            load_game_scene: false,
            is_done: false,
        }
    }

    fn draw_title(&self, canvas: &mut WindowCanvas, texture_creator: &TextureCreator<sdl2::video::WindowContext>, font: &Font) {
        let title_text = font
            .render("Space Invaders!")
            .solid(Colour::RGB(255, 255, 255))
            .unwrap();
        let title_texture = texture_creator
            .create_texture_from_surface(title_text)
            .unwrap();

        let title_texture_data = title_texture.query();

        canvas
            .copy(
                &title_texture,
                None,
                sdl2::rect::Rect::from_center(
                    sdl2::rect::Point::new(canvas.viewport().width() as i32 / 2, 128),
                    title_texture_data.width,
                    title_texture_data.height,
                ),
            )
            .unwrap();
    }
}

impl Scene for MainMenuScene {
    fn is_done(&self) -> bool {
        self.is_done
    }

    fn on_load(&mut self, _canvas: &WindowCanvas) -> (Vec<String>, Vec<String>) {
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
        if input_state.is_key_down(Scancode::Return) {
            self.load_game_scene = true;
        } else if input_state.is_key_down(Scancode::Escape) {
            self.is_done = true;
        }
    }

    fn update(
        &mut self,
        _delta_time: f32,
        scene_queue: &mut VecDeque<Box<dyn Scene>>,
        _canvas: &WindowCanvas,
    ) {
        if self.load_game_scene {
            self.is_done = true;
            scene_queue.push_back(Box::new(SpaceScene::new()));
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

        self.draw_title(canvas, texture_creator, &fonts[self.font_index]);
    }
}
