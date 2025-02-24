use glam::{Mat4, Vec3};
use crate::settings::Settings;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub up: Vec3,
    pub right: Vec3,
    pub forward: Vec3,
    pub m_proj: Mat4,
    pub m_view: Mat4,
    pitch_max: f32
}

impl Camera {
    pub fn new(settings: &Settings) -> Self {
        let mut camera = Self {
            position: settings.player_pos,
            yaw: (-90.0f32).to_radians(),
            pitch: 0.0,
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            forward: Vec3::new(0.0, 0.0, -1.0),
            m_proj: Mat4::perspective_rh(settings.v_fov, settings.aspect_ratio, settings.near, settings.far),
            m_view: Mat4::IDENTITY,
            pitch_max: settings.pitch_max
        };
        camera.update();
        camera
    }

    pub fn update(&mut self) {
        self.update_vectors();
        self.update_view_matrix();
    }

    fn update_view_matrix(&mut self) {
        self.m_view = Mat4::look_at_rh(self.position, self.position + self.forward, self.up);
    }

    fn update_vectors(&mut self) {
        self.forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize();
        self.right = self.forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(self.forward).normalize();
    }

    pub fn rotate_pitch(&mut self, delta_y: f32) {
        self.pitch = (self.pitch - delta_y).clamp(-self.pitch_max, self.pitch_max);
    }

    pub fn rotate_yaw(&mut self, delta_x: f32) {
        self.yaw += delta_x;
    }

    pub fn move_left(&mut self, velocity: f32) {
        self.position -= self.right * velocity;
    }

    pub fn move_right(&mut self, velocity: f32) {
        self.position += self.right * velocity;
    }

    pub fn move_up(&mut self, velocity: f32) {
        self.position += self.up * velocity;
    }

    pub fn move_down(&mut self, velocity: f32) {
        self.position -= self.up * velocity;
    }

    pub fn move_forward(&mut self, velocity: f32) {
        self.position += self.forward * velocity;
    }

    pub fn move_back(&mut self, velocity: f32) {
        self.position -= self.forward * velocity;
    }
}