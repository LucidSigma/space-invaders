use std::collections::VecDeque;

use sdl2::keyboard::Scancode;
use sdl2::pixels::Color as Colour;
use sdl2::render::{Texture, WindowCanvas};

use crate::game::input::InputState;
use crate::game::scene::Scene;
use crate::scenes::space_scene::SpaceScene;

const BACKGROUND_COLOUR: Colour = Colour::RGB(10, 10, 10);

pub struct MainMenuScene {
    load_game_scene: bool,
    is_done: bool,
}

impl MainMenuScene {
    pub fn new() -> MainMenuScene {
        MainMenuScene {
            load_game_scene: false,
            is_done: false,
        }
    }
}

impl Scene for MainMenuScene {
    fn is_done(&self) -> bool {
        self.is_done
    }

    fn on_load(&mut self, _canvas: &WindowCanvas) -> Vec<String> {
        vec![]
    }

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

    fn draw(&mut self, canvas: &mut WindowCanvas, _textures: &[Texture]) {
        canvas.set_draw_color(BACKGROUND_COLOUR);
        canvas.clear();
    }
}
