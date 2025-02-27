use crate::player::Player;
use crate::settings::{CHUNK_AREA, CHUNK_SIZE, CHUNK_VOL, MIN_Y, Settings, MAX_Y};
use crate::shader_program::ShaderProgram;
use crate::world_objects::Chunk;
use glam::IVec3;
use noise::{NoiseFn, Simplex};
use rand::Rng;
use std::collections::HashMap;

pub struct World {
    pub(crate) chunks: HashMap<IVec3, Chunk>,
    shader_program: ShaderProgram,
    render_distance: i32,
    simplex: Simplex, // Додаємо Simplex для генерації шуму
}

impl World {
    pub fn new(shader_program: &ShaderProgram, settings: &Settings) -> Self {
        let mut world = Self {
            chunks: HashMap::new(),
            shader_program: shader_program.clone(),
            render_distance: settings.render_distance,
            simplex: Simplex::new(settings.world_seed), // Ініціалізація Simplex із фіксованим seed (можна зробити конфігурованим)
        };
        world.build_chunks(IVec3::ZERO);
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
        // self.chunks.retain(|pos, _| {
        //     let dist = (pos - center_pos).abs().max_element();
        //     dist <= self.render_distance
        // });

        // for x in -self.render_distance..=self.render_distance {
        //     for y in -self.render_distance..=self.render_distance {
        //         for z in -self.render_distance..=self.render_distance {
        //             let chunk_pos = center_pos + IVec3::new(x, y, z);
        //             if !self.chunks.contains_key(&chunk_pos) {
        //                 let voxels = self.generate_voxels(chunk_pos);
        //                 let chunk = Chunk::new(&self.shader_program, chunk_pos, voxels, self);
        //                 self.chunks.insert(chunk_pos, chunk);
        //             }
        //         }
        //     }
        // }

        // Видаляємо чанки поза межами render_distance або нижче MIN_Y
        // Видаляємо чанки поза межами render_distance або за межами MIN_Y/MAX_Y
        self.chunks.retain(|pos, _| {
            let rel_x = pos.x - center_pos.x;
            let rel_y = pos.y - center_pos.y;
            let rel_z = pos.z - center_pos.z;
            rel_x >= -self.render_distance &&
                rel_x <= self.render_distance &&
                rel_y >= (MIN_Y - center_pos.y).max(-self.render_distance) && // Нижня межа
                rel_y <= (MAX_Y - center_pos.y).min(self.render_distance) && // Верхня межа
                rel_z >= -self.render_distance &&
                rel_z <= self.render_distance &&
                pos.y >= MIN_Y && // Абсолютне обмеження по MIN_Y
                pos.y <= MAX_Y // Абсолютне обмеження по MAX_Y
        });

        // Генеруємо чанки в межах render_distance, але між MIN_Y і MAX_Y
        for x in -self.render_distance..=self.render_distance {
            for y in (MIN_Y - center_pos.y).max(-self.render_distance)
                ..=(MAX_Y - center_pos.y).min(self.render_distance)
            {
                for z in -self.render_distance..=self.render_distance {
                    let chunk_pos = center_pos + IVec3::new(x, y, z);
                    if chunk_pos.y >= MIN_Y
                        && chunk_pos.y <= MAX_Y
                        && !self.chunks.contains_key(&chunk_pos)
                    {
                        let voxels = self.generate_voxels(chunk_pos);
                        let chunk = Chunk::new(&self.shader_program, chunk_pos, voxels, self);
                        self.chunks.insert(chunk_pos, chunk);
                    }
                }
            }
        }
    }

    // Генерація вокселів перенесена в World
    fn generate_voxels(&self, position: IVec3) -> Vec<u8> {
        let mut voxels = vec![0u8; CHUNK_VOL as usize];
        let simplex = Simplex::new(0); // Можна передати сід із World, якщо потрібно
        let (cx, cy, cz) = (position.x, position.y, position.z);

        let rng = rand::thread_rng().gen_range(1..=100);

        for x in 0..CHUNK_SIZE {
            let wx = x as f32 + cx as f32 * CHUNK_SIZE as f32;
            for z in 0..CHUNK_SIZE {
                let wz = z as f32 + cz as f32 * CHUNK_SIZE as f32;
                let noise_value = simplex.get([wx as f64 * 0.01, wz as f64 * 0.01]);
                let world_height = (noise_value * 32.0 + 32.0) as i32;
                // Висота поверхні в межах MIN_Y..MAX_Y
                // let world_height = MIN_Y * CHUNK_SIZE as i32 + ((noise_value + 32.0) * 0.5 * (MAX_Y - MIN_Y) as f64 * CHUNK_SIZE as f64) as i32;
                let local_height =
                    (world_height - cy * CHUNK_SIZE as i32).clamp(0, CHUNK_SIZE as i32);

                for y in 0..local_height as u32 {
                    let wy = y as f32 + cy as f32 * CHUNK_SIZE as f32;
                    voxels[(x + CHUNK_SIZE * z + CHUNK_AREA * y) as usize] = rng as u8;
                }
            }
        }
        voxels
    }
}
