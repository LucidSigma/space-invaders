use std::collections::VecDeque;
use std::fs;

use sdl2::{
    keyboard::Scancode,
    mixer::{Channel, Chunk, Music},
    mouse::MouseButton,
    pixels::Color as Colour,
    rect::{Point, Rect},
    render::{Texture, TextureCreator, WindowCanvas},
    ttf::Font,
};

use super::button::Button;
use crate::game::input::InputState;
use crate::game::scene::Scene;
use crate::scenes::space_scene::SpaceScene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(10, 10, 10);

pub struct MainMenuScene<'a> {
    font_index: usize,
    buttons: Vec<Button<'a>>,

    is_done: bool,

    button_hover_sound: Option<Chunk>,
    button_select_sound: Option<Chunk>,

    music: Option<Music<'a>>,
}

impl<'a> MainMenuScene<'a> {
    pub fn new() -> MainMenuScene<'a> {
        MainMenuScene {
            font_index: 0,
            buttons: vec![],
            is_done: false,
            button_hover_sound: None,
            button_select_sound: None,
            music: None,
        }
    }

    fn draw_title(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
        font: &Font,
    ) {
        let title_text = font.render("Space Invaders!").solid(Colour::WHITE).unwrap();
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
                    title_texture_data.width,
                    title_texture_data.height,
                ),
            )
            .unwrap();
    }
}

impl Scene for MainMenuScene<'_> {
    fn is_done(&self) -> bool {
        self.is_done
    }

    fn on_load(
        &mut self,
        _canvas: &WindowCanvas,
        _previous_scene_payload: Option<i32>,
    ) -> (Vec<String>, Vec<String>) {
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

        self.music = Some(Music::from_file("assets/sounds/music/Chill Wave.mp3").unwrap());

        (vec![], fonts)
    }

    fn on_late_load(&mut self, canvas: &WindowCanvas, _textures: &[Texture], _fonts: &[Font]) {
        self.buttons.push(Button::new(
            canvas.viewport().width() / 2,
            canvas.viewport().height() / 2,
            400,
            150,
            "Play",
            0.75,
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
            400,
            150,
            "Quit",
            0.75,
        ));
        self.buttons.last_mut().unwrap().set_colours(
            Colour::BLACK,
            Colour::BLACK,
            Colour::BLACK,
            Colour::YELLOW,
            Colour::GREEN,
            Colour::RED,
        );

        self.music.as_ref().unwrap().play(-1).unwrap();
    }

    fn on_unload(&mut self) -> Option<i32> {
        Music::halt();

        None
    }

    fn poll_event(&mut self, event: sdl2::event::Event) {
        use sdl2::event::Event::*;
        use sdl2::event::WindowEvent::{Minimized as Minimised, *};

        if let Window {
            win_event: window_event,
            ..
        } = event
        {
            match window_event {
                FocusGained | Restored => {
                    if Music::is_paused() {
                        Music::resume();
                    }
                }
                FocusLost | Minimised => {
                    if Music::is_playing() {
                        Music::pause();
                    }
                }
                _ => (),
            }
        }
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

        self.draw_title(canvas, texture_creator, &fonts[self.font_index]);

        for button in &self.buttons {
            button.draw(canvas, texture_creator, &fonts[self.font_index]);
        }
    }
}
