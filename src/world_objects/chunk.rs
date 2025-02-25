use crate::meshes::chunk_mesh::ChunkMesh;
use crate::settings::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOL};
use crate::shader_program::ShaderProgram;
use glam::Vec3;
use noise::{NoiseFn, Perlin, Simplex};

pub struct Chunk {
    pub voxels: Vec<u8>,
    pub mesh: Option<ChunkMesh>, // Змінено на Option для відтворення Python-логіки
    pub shader_program: ShaderProgram,
}

impl Chunk {
    fn build_voxels() -> Vec<u8> {
        // Порожній чанк
        let mut voxels = vec![0u8; CHUNK_VOL as usize];

        // Ініціалізація генератора шуму Simplex
        // let simplex = Simplex::new(0); // Seed 0, як приклад
        // let perlin = Perlin::new(5);

        let simplex = Simplex::new(8);

        let angle = 90.0f32.to_radians(); // Поворот на 45°
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        // Заповнення чанка з використанням циклів
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let pos = Vec3::new(x as f32, y as f32, z as f32) * 0.1;

                    // voxels[(x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize] = (x + y + z) as u8;
                    voxels[(x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize] =
                        if (simplex.get([pos.x as f64, pos.y as f64, pos.z as f64]) + 1.0) as i64
                            > 0
                        {
                            (x + y + z) as u8
                        } else {
                            0
                        }
                }
            }
        }
        voxels
    }

    fn build_mesh(&mut self) {
        // Ініціалізація mesh як у Python
        self.mesh = Some(ChunkMesh::new(self));
    }

    pub fn render(&self) {
        // Переконуємося, що mesh ініціалізований перед рендерингом
        if let Some(mesh) = &self.mesh {
            mesh.render();
        }
    }

    pub fn new(shader_program: &ShaderProgram) -> Self {
        let voxels = Self::build_voxels();
        let mut chunk = Self {
            voxels,
            mesh: None,                             // Спочатку None, як у Python
            shader_program: shader_program.clone(), // Клонуємо ShaderProgram
        };
        chunk.build_mesh(); // Викликаємо build_mesh для ініціалізації mesh
        chunk
    }
}
