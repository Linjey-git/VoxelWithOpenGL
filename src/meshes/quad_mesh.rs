use crate::meshes::base_mesh::BaseMesh;
use crate::shader_program::ShaderProgram;

pub struct QuadMesh {
    base: BaseMesh,
}

impl QuadMesh {
    pub fn new(shader_program: &ShaderProgram) -> Self {
        // Розділені позиції (аналог vertices у Python)
        let positions: [f32; 18] = [
            0.5, 0.5, 0.0,   // Вершина 1
            -0.5, 0.5, 0.0,  // Вершина 2
            -0.5, -0.5, 0.0, // Вершина 3
            0.5, 0.5, 0.0,   // Вершина 4
            -0.5, -0.5, 0.0, // Вершина 5
            0.5, -0.5, 0.0,  // Вершина 6
        ];

        // Розділені кольори (аналог colors у Python)
        let colors: [f32; 18] = [
            0.0, 1.0, 0.0, // Зелений
            1.0, 0.0, 0.0, // Червоний
            1.0, 1.0, 0.0, // Жовтий
            0.0, 1.0, 0.0, // Зелений
            1.0, 1.0, 0.0, // Жовтий
            0.0, 0.0, 1.0, // Синій
        ];

        // Об'єднання позицій і кольорів у єдиний масив (аналог np.hstack)
        let mut vertex_data = Vec::with_capacity(36);
        for i in 0..6 {
            vertex_data.extend_from_slice(&positions[i * 3..i * 3 + 3]); // Додаємо позицію
            vertex_data.extend_from_slice(&colors[i * 3..i * 3 + 3]);    // Додаємо колір
        }

        let attrs = [(0, 3), (1, 3)]; // Позиція (3f) + колір (3f)
        Self {
            base: BaseMesh::new(shader_program.quad_program(), &vertex_data, &attrs, 6),
        }
    }

    pub fn render(&self) {
        self.base.render();
    }
}