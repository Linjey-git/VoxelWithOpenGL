use glam::{Vec2, Vec3};

#[derive(Debug)]
pub struct Settings {
    pub win_res: Vec2,          // Роздільна здатність вікна
    pub aspect_ratio: f32,      // Співвідношення сторін
    pub fov_deg: f32,           // Поле зору в градусах
    pub v_fov: f32,             // Вертикальний FOV у радіанах
    pub h_fov: f32,             // Горизонтальний FOV у радіанах
    pub near: f32,              // Ближня площина відсікання
    pub far: f32,               // Дальня площина відсікання
    pub pitch_max: f32,         // Максимальний кут pitch
    pub player_speed: f32,      // Швидкість руху гравця
    pub player_rot_speed: f32,  // Швидкість повороту гравця
    pub player_pos: Vec3,       // Початкова позиція гравця
    pub mouse_sensitivity: f32, // Чутливість миші
    pub bg_color: Vec3,         // Колір фону
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
            aspect_ratio,
            fov_deg,
            v_fov,
            h_fov,
            near: 0.1,
            far: 2000.0,
            pitch_max: 89.0f32.to_radians(),
            player_speed: 0.005,
            player_rot_speed: 0.003,
            player_pos: Vec3::new(0.0, 0.0, 1.0),
            mouse_sensitivity: 0.002,
            bg_color: Vec3::new(0.1, 0.16, 0.25),
        }
    }

    // Метод для оновлення залежних значень при зміні win_res чи fov_deg
    pub fn update_dependent(&mut self) {
        self.aspect_ratio = self.win_res.x / self.win_res.y;
        self.v_fov = self.fov_deg.to_radians();
        self.h_fov = 2.0 * (self.v_fov * 0.5).tan().atan2(self.aspect_ratio);
    }

    // Методи для зміни налаштувань із меню
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
