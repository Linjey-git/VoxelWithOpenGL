use glam::{Vec2, Vec3};

pub const CHUNK_SIZE: u32 = 32;
pub const H_CHUNK_SIZE: u32 = CHUNK_SIZE / 2;
pub const CHUNK_AREA: u32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOL: u32 = CHUNK_AREA * CHUNK_SIZE;
pub const MIN_Y: i32 = -2; // Нижня межа світу в чанках
pub const MAX_Y: i32 = 2; // Верхня межа світу

#[derive(Debug)]
pub struct Settings {
    pub win_res: Vec2,
    pub chunk_size: u32,
    pub h_chunk_size: u32,
    pub chunk_area: u32,
    pub chunk_vol: u32,
    pub aspect_ratio: f32,
    pub fov_deg: f32,
    pub v_fov: f32,
    pub h_fov: f32,
    pub near: f32,
    pub far: f32,
    pub pitch_max: f32,
    pub player_speed: f32,
    pub player_rot_speed: f32,
    pub player_pos: Vec3,
    pub mouse_sensitivity: f32,
    pub bg_color: Vec3,
    pub render_distance: i32, // Додано
    pub world_seed: u32,      // Додано
}

impl Settings {
    pub fn new() -> Self {
        let win_res = Vec2::new(1600.0, 900.0);
        let aspect_ratio = win_res.x / win_res.y;
        let fov_deg: f32 = 50.0;
        let v_fov = fov_deg.to_radians();
        let h_fov = 2.0 * (v_fov * 0.5).tan().atan2(aspect_ratio);

        Self {
            win_res,
            chunk_size: CHUNK_SIZE,
            h_chunk_size: H_CHUNK_SIZE,
            chunk_area: CHUNK_AREA,
            chunk_vol: CHUNK_VOL,
            aspect_ratio,
            fov_deg,
            v_fov,
            h_fov,
            near: 0.1,
            far: 2000.0,
            pitch_max: 89.0f32.to_radians(),
            player_speed: 0.025, //0.005
            player_rot_speed: 0.003,
            player_pos: Vec3::new(H_CHUNK_SIZE as f32 + 15.0, CHUNK_SIZE as f32, 1.5 * CHUNK_SIZE as f32),
            mouse_sensitivity: 0.002,
            bg_color: Vec3::new(0.1, 0.16, 0.25),
            render_distance: 3, // Значення за замовчуванням
            world_seed: 0,      // Значення за замовчуванням
        }
    }

    pub fn update_dependent(&mut self) {
        self.aspect_ratio = self.win_res.x / self.win_res.y;
        self.v_fov = self.fov_deg.to_radians();
        self.h_fov = 2.0 * (self.v_fov * 0.5).tan().atan2(self.aspect_ratio);
    }

    pub fn set_resolution(&mut self, width: f32, height: f32) {
        self.win_res = Vec2::new(width, height);
        self.update_dependent();
    }

    pub fn set_fov(&mut self, fov_deg: f32) {
        self.fov_deg = fov_deg;
        self.update_dependent();
    }

    pub fn set_mouse_sensitivity(&mut self, sensitivity: f32) {
        self.mouse_sensitivity = sensitivity;
    }
}