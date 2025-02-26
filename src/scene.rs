use crate::shader_program::ShaderProgram;
use crate::world::World;
use crate::settings::Settings;
use crate::player::Player;

pub struct Scene {
    world: World,
}

impl Scene {
    pub fn new(shader_program: &ShaderProgram, settings: &Settings) -> Self {
        Self {
            world: World::new(shader_program, settings),
        }
    }

    pub fn update(&mut self, player: &Player) {
        self.world.update(player);
    }

    pub fn render(&self) {
        self.world.render();
    }
}