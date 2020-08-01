use std::collections::VecDeque;
use std::fs;

use sdl2::{
    keyboard::Scancode,
    mixer::{Channel, Chunk},
    mouse::MouseButton,
    pixels::Color as Colour,
    rect::{Point, Rect},
    render::{Texture, TextureCreator, WindowCanvas},
    ttf::Font,
};

use super::button::Button;
use crate::game::input::InputState;
use crate::game::scene::Scene;
use crate::scenes::main_menu_scene::MainMenuScene;
use crate::scenes::space_scene::SpaceScene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(10, 10, 10);

pub struct GameOverScene<'a> {
    player_score: u32,

    buttons: Vec<Button<'a>>,
    button_hover_sound: Option<Chunk>,
    button_select_sound: Option<Chunk>,

    is_done: bool,
    font_index: usize,
}

impl<'a> GameOverScene<'a> {
    pub fn new() -> GameOverScene<'a> {
        GameOverScene {
            player_score: 0,
            buttons: vec![],
            button_hover_sound: None,
            button_select_sound: None,
            is_done: false,
            font_index: 0,
        }
    }

    fn draw_header(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
        font: &Font,
    ) {
        const TEXT_SCALE: f32 = 0.75;

        let title_text = font.render("Game Over!").solid(Colour::WHITE).unwrap();
        let title_texture = texture_creator
            .create_texture_from_surface(title_text)
            .unwrap();

        let title_texture_data = title_texture.query();

        canvas
            .copy(
                &title_texture,
                None,
                Rect::from_center(
                    Point::new(canvas.viewport().width() as i32 / 2, 128),
                    (title_texture_data.width as f32 * TEXT_SCALE) as u32,
                    (title_texture_data.height as f32 * TEXT_SCALE) as u32,
                ),
            )
            .unwrap();
    }

    fn draw_score_text(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
        font: &Font,
    ) {
        const TEXT_SCALE: f32 = 0.25;

        let title_text = font
            .render(format!("Your final score was {}.", self.player_score).as_str())
            .solid(Colour::YELLOW)
            .unwrap();
        let title_texture = texture_creator
            .create_texture_from_surface(title_text)
            .unwrap();

        let title_texture_data = title_texture.query();

        canvas
            .copy(
                &title_texture,
                None,
                Rect::from_center(
                    Point::new(canvas.viewport().width() as i32 / 2, 256),
                    (title_texture_data.width as f32 * TEXT_SCALE) as u32,
                    (title_texture_data.height as f32 * TEXT_SCALE) as u32,
                ),
            )
            .unwrap();
    }
}

impl Scene for GameOverScene<'_> {
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

        for sound_file in fs::read_dir("assets/sounds/effects/menu").unwrap() {
            let sound_file = sound_file.unwrap();
            let sound_filepath = sound_file.path();
            let sound_filepath_string = sound_filepath
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();

            let loaded_sound_chunk = Some(Chunk::from_file(sound_filepath).unwrap());

            match sound_filepath_string.as_ref() {
                "button_hover.wav" => {
                    self.button_hover_sound = loaded_sound_chunk;
                }
                "button_select.wav" => {
                    self.button_select_sound = loaded_sound_chunk;
                }
                _ => (),
            }
        }

        (vec![], fonts)
    }

    fn on_late_load(&mut self, canvas: &WindowCanvas, _textures: &[Texture], _fonts: &[Font]) {
        self.buttons.push(Button::new(
            canvas.viewport().width() / 2,
            canvas.viewport().height() / 2,
            450,
            125,
            "Play Again",
            0.5,
        ));
        self.buttons.last_mut().unwrap().set_colours(
            Colour::BLACK,
            Colour::BLACK,
            Colour::BLACK,
            Colour::YELLOW,
            Colour::GREEN,
            Colour::WHITE,
        );

        self.buttons.push(Button::new(
            canvas.viewport().width() / 2,
            canvas.viewport().height() / 4 * 3,
            450,
            125,
            "Back to Menu",
            0.5,
        ));
        self.buttons.last_mut().unwrap().set_colours(
            Colour::BLACK,
            Colour::BLACK,
            Colour::BLACK,
            Colour::YELLOW,
            Colour::GREEN,
            Colour::RED,
        );
    }

    fn process_input(&mut self, input_state: &InputState) {
        if input_state.is_key_down(Scancode::Escape) {
            self.is_done = true;
        }

        for button in &mut self.buttons {
            button.is_hovered = false;
            button.is_clicked = false;

            if button.is_mouse_over(input_state) {
                button.is_hovered = true;

                if input_state.is_mouse_button_down(MouseButton::Left) {
                    button.is_clicked = true;
                }
            } else if button.played_enter_sound {
                button.played_enter_sound = false;
            }
        }
    }

    fn update(
        &mut self,
        _delta_time: f32,
        scene_queue: &mut VecDeque<Box<dyn Scene>>,
        _canvas: &WindowCanvas,
        sound_channel: &Channel,
    ) {
        if self.is_done {
            scene_queue.push_back(Box::new(MainMenuScene::new()));

            return;
        }

        for button in &mut self.buttons {
            if button.is_hovered && !button.played_enter_sound {
                sound_channel
                    .play(self.button_hover_sound.as_ref().unwrap(), 0)
                    .unwrap();
                button.played_enter_sound = true;
            }
        }

        if self.buttons.first().unwrap().is_clicked {
            self.is_done = true;
            scene_queue.push_back(Box::new(SpaceScene::new()));

            sound_channel
                .play(self.button_select_sound.as_ref().unwrap(), 0)
                .unwrap();
        }

        if self.buttons.last().unwrap().is_clicked {
            self.is_done = true;
            scene_queue.push_back(Box::new(MainMenuScene::new()));

            sound_channel
                .play(self.button_select_sound.as_ref().unwrap(), 0)
                .unwrap();
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

        self.draw_header(canvas, texture_creator, &fonts[self.font_index]);
        self.draw_score_text(canvas, texture_creator, &fonts[self.font_index]);

        for button in &self.buttons {
            button.draw(canvas, texture_creator, &fonts[self.font_index]);
        }
    }
}
