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

struct Button<'a> {
    rect: sdl2::rect::Rect,
    midpoint: (u32, u32),

    text: &'a str,
    text_scale: f32,

    is_hovered: bool,
    is_clicked: bool,

    text_colour: Colour,
    hovered_text_colour: Colour,
    clicked_text_colour: Colour,

    background_colour: Colour,
    hovered_background_colour: Colour,
    clicked_background_colour: Colour,
}

impl Button<'_> {
    fn new(x: u32, y: u32, width: u32, height: u32, text: &str, text_scale: f32) -> Button {
        Button {
            rect: sdl2::rect::Rect::from_center(
                sdl2::rect::Point::new(x as i32, y as i32),
                width,
                height,
            ),
            midpoint: (x, y),
            text,
            text_scale,
            is_hovered: false,
            is_clicked: false,
            text_colour: Colour::BLACK,
            hovered_text_colour: Colour::BLACK,
            clicked_text_colour: Colour::BLACK,
            background_colour: Colour::WHITE,
            hovered_background_colour: Colour::WHITE,
            clicked_background_colour: Colour::WHITE,
        }
    }

    fn set_colours(
        &mut self,
        text_colour: Colour,
        hovered_text_colour: Colour,
        clicked_text_colour: Colour,
        background_colour: Colour,
        hovered_background_colour: Colour,
        clicked_background_colour: Colour,
    ) {
        self.text_colour = text_colour;
        self.hovered_text_colour = hovered_text_colour;
        self.clicked_text_colour = clicked_text_colour;
        self.background_colour = background_colour;
        self.hovered_background_colour = hovered_background_colour;
        self.clicked_background_colour = clicked_background_colour;
    }

    fn is_mouse_over(&self, input_state: &InputState) -> bool {
        self.rect.contains_point(sdl2::rect::Point::new(
            input_state.mouse_x,
            input_state.mouse_y,
        ))
    }

    fn draw(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
        font: &Font,
    ) {
        let (text_colour, background_colour) = if self.is_clicked {
            (self.clicked_text_colour, self.clicked_background_colour)
        } else if self.is_hovered {
            (self.hovered_text_colour, self.hovered_background_colour)
        } else {
            (self.text_colour, self.background_colour)
        };

        canvas.set_draw_color(background_colour);
        canvas.fill_rect(self.rect).unwrap();

        let text = font.render(self.text).solid(text_colour).unwrap();

        let text_texture = texture_creator.create_texture_from_surface(text).unwrap();

        let text_texture_data = text_texture.query();

        canvas
            .copy(
                &text_texture,
                None,
                sdl2::rect::Rect::from_center(
                    sdl2::rect::Point::new(self.midpoint.0 as i32, self.midpoint.1 as i32),
                    (text_texture_data.width as f32 * self.text_scale) as u32,
                    (text_texture_data.height as f32 * self.text_scale) as u32,
                ),
            )
            .unwrap();
    }
}

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
                sdl2::rect::Rect::from_center(
                    sdl2::rect::Point::new(canvas.viewport().width() as i32 / 2, 128),
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

                if input_state.is_mouse_button_down(sdl2::mouse::MouseButton::Left) {
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