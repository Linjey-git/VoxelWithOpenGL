use crate::camera::Camera;
use crate::settings::Settings;
use sdl2::EventPump;

pub struct Player {
    pub(crate) camera: Camera,
}

impl Player {
    pub fn new(settings: &Settings) -> Self {
        Self {
            camera: Camera::new(settings),
        }
    }

    pub fn update(&mut self, event_pump: &mut EventPump, delta_time: f32, settings: &Settings) {
        self.keyboard_control(event_pump, delta_time, settings);
        self.mouse_control(event_pump, settings);
        self.camera.update();
    }

    fn mouse_control(&mut self, event_pump: &mut EventPump, settings: &Settings) {
        let mouse_state = event_pump.relative_mouse_state();
        let mouse_dx = mouse_state.x() as f32;
        let mouse_dy = mouse_state.y() as f32;

        if mouse_dx != 0.0 {
            self.camera
                .rotate_yaw(mouse_dx * settings.mouse_sensitivity);
        }
        if mouse_dy != 0.0 {
            self.camera
                .rotate_pitch(mouse_dy * settings.mouse_sensitivity);
        }
    }

    fn keyboard_control(&mut self, event_pump: &mut EventPump, delta_time: f32, settings: &Settings) {
        let key_state = event_pump.keyboard_state();
        let vel = settings.player_speed * delta_time;

        if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            self.camera.move_forward(vel);
        }
        if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            self.camera.move_back(vel);
        }
        if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            self.camera.move_right(vel);
        }
        if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            self.camera.move_left(vel);
        }
        if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::Space) {
            self.camera.move_up(vel);
        }
        if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::LShift) {
            self.camera.move_down(vel);
        }
    }

    pub fn m_view(&self) -> &glam::Mat4 {
        &self.camera.m_view
    }
    pub fn m_proj(&self) -> &glam::Mat4 {
        &self.camera.m_proj
    }
}
