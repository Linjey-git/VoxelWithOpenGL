use crate::shader_program::ShaderProgram;
use crate::world_objects::Chunk;

pub struct Scene {
    chunk: Chunk,
}

impl Scene {
    pub fn new(shader_program: &ShaderProgram) -> Self {
        Self {
            chunk: Chunk::new(shader_program),
        }
    }

    pub fn update(&mut self) {
        // Поки порожній, як у Python
    }

    pub fn render(&self) {
        self.chunk.render();
    }
}