mod game;
mod scenes;

use crate::scenes::space_scene::SpaceScene;

fn main() {
    game::play(Box::new(SpaceScene::new()));
}
