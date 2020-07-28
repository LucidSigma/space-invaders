mod game;
mod scenes;

use crate::scenes::main_menu_scene::MainMenuScene;

fn main() {
    game::play(Box::new(MainMenuScene::new()));
}
