use crate::player::Player;
use crate::settings::{CHUNK_SIZE, Settings};
use crate::shader_program::ShaderProgram;
use crate::world_objects::Chunk;
use glam::IVec3;
use std::collections::HashMap;

pub struct World {
    pub(crate) chunks: HashMap<IVec3, Chunk>,
    shader_program: ShaderProgram,
    render_distance: i32,
}

impl World {
    pub fn new(shader_program: &ShaderProgram, settings: &Settings) -> Self {
        let mut world = Self {
            chunks: HashMap::new(),
            shader_program: shader_program.clone(),
            render_distance: settings.render_distance,
        };
        world.build_chunks(IVec3::ZERO); // Початкова генерація навколо (0, 0, 0)
        world
    }

    pub fn update(&mut self, player: &Player) {
        let player_chunk_pos = Self::world_to_chunk_pos(player.camera.position);
        self.build_chunks(player_chunk_pos);
    }

    pub fn render(&self) {
        for chunk in self.chunks.values() {
            chunk.render();
        }
    }

    fn world_to_chunk_pos(pos: glam::Vec3) -> IVec3 {
        IVec3::new(
            (pos.x / CHUNK_SIZE as f32).floor() as i32,
            (pos.y / CHUNK_SIZE as f32).floor() as i32,
            (pos.z / CHUNK_SIZE as f32).floor() as i32,
        )
    }

    fn build_chunks(&mut self, center_pos: IVec3) {
        self.chunks.retain(|pos, _| {
            let dist = (pos - center_pos).abs().max_element();
            dist <= self.render_distance
        });

        for x in -self.render_distance..=self.render_distance {
            for y in -self.render_distance..=self.render_distance {
                for z in -self.render_distance..=self.render_distance {
                    let chunk_pos = center_pos + IVec3::new(x, y, z);
                    if !self.chunks.contains_key(&chunk_pos) {
                        let chunk = Chunk::new(&self.shader_program, chunk_pos, self);
                        self.chunks.insert(chunk_pos, chunk);
                    }
                }
            }
        }
    }

    fn generate_chunk(&self, position: IVec3) -> Chunk {
        Chunk::new(&self.shader_program, position, self)
    }
}