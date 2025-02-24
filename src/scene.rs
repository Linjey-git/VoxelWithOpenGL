use crate::shader_program::ShaderProgram;
use crate::meshes::QuadMesh;

pub struct Scene {
    quad: QuadMesh,
}

impl Scene {
    pub fn new(shader_program: &ShaderProgram) -> Self {
        Self {
            quad: QuadMesh::new(shader_program),
        }
    }

    pub fn update(&mut self) {
        // Поки порожній, як у Python
    }

    pub fn render(&self) {
        self.quad.render();
    }
}