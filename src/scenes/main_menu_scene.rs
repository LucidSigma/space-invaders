mod button;

use std::collections::VecDeque;
use std::fs;

use sdl2::{
    keyboard::Scancode,
    mouse::MouseButton,
    pixels::Color as Colour,
    rect::{Point, Rect},
    render::{Texture, TextureCreator, WindowCanvas},
    ttf::Font,
};

use self::button::Button;
use crate::game::input::InputState;
use crate::game::scene::Scene;
use crate::scenes::space_scene::SpaceScene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(10, 10, 10);

pub struct MainMenuScene<'a> {
    font_index: usize,
    buttons: Vec<Button<'a>>,

    is_done: bool,
}

impl<'a> MainMenuScene<'a> {
    pub fn new() -> MainMenuScene<'a> {
        MainMenuScene {
            font_index: 0,
            buttons: vec![],
            is_done: false,
        }
    }

    fn draw_title(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
        font: &Font,
    ) {
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
            }
        }
    }

    fn update(
        &mut self,
        _delta_time: f32,
        scene_queue: &mut VecDeque<Box<dyn Scene>>,
        _canvas: &WindowCanvas,
        _sound_channel: &sdl2::mixer::Channel,
    ) {
        if self.buttons.first().unwrap().is_clicked {
            self.is_done = true;
            scene_queue.push_back(Box::new(SpaceScene::new()));
        }

        if self.buttons.last().unwrap().is_clicked {
            self.is_done = true;
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
