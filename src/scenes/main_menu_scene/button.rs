use sdl2::pixels::Color as Colour;
use sdl2::rect::{Point, Rect};

use crate::game::input::InputState;

pub struct Button<'a> {
    rect: Rect,
    midpoint: (u32, u32),

    pub text: &'a str,
    pub text_scale: f32,

    pub is_hovered: bool,
    pub is_clicked: bool,

    text_colour: Colour,
    hovered_text_colour: Colour,
    clicked_text_colour: Colour,

    background_colour: Colour,
    hovered_background_colour: Colour,
    clicked_background_colour: Colour,
}

impl Button<'_> {
    pub fn new(x: u32, y: u32, width: u32, height: u32, text: &str, text_scale: f32) -> Button {
        Button {
            rect: Rect::from_center(Point::new(x as i32, y as i32), width, height),
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

    #[allow(dead_code)]
    pub fn set_position(&mut self, x: u32, y: u32, width: Option<u32>, height: Option<u32>) {
        let width = if let Some(width) = width {
            width
        } else {
            self.rect.width()
        };

        let height = if let Some(height) = height {
            height
        } else {
            self.rect.height()
        };

        self.rect = Rect::from_center(Point::new(x as i32, y as i32), width, height);
        self.midpoint = (x, y);
    }

    pub fn set_colours(
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

    pub fn is_mouse_over(&self, input_state: &InputState) -> bool {
        self.rect
            .contains_point(Point::new(input_state.mouse_x, input_state.mouse_y))
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::WindowCanvas,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font: &sdl2::ttf::Font,
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
                Rect::from_center(
                    Point::new(self.midpoint.0 as i32, self.midpoint.1 as i32),
                    (text_texture_data.width as f32 * self.text_scale) as u32,
                    (text_texture_data.height as f32 * self.text_scale) as u32,
                ),
            )
            .unwrap();
    }
}
