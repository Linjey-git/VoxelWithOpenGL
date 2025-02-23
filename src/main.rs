mod settings;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::{GLProfile, Window};
use std::time::Instant;
use crate::settings::{WIN_RES, BG_COLOR};

struct VoxelEngine {
    sdl_context: sdl2::Sdl,        // Основний контекст SDL2
    window: Window,                // Вікно SDL2
    last_frame: Instant,           // Час останнього кадру для delta_time
    frame_count: u32,              // Лічильник кадрів для FPS
    fps_timer: f32,                // Таймер для оновлення FPS
    is_running: bool,              // Прапорець для головного циклу
    event_pump: sdl2::EventPump,   // Обробник подій SDL2
}

impl VoxelEngine {
    fn new() -> Self {
        // Ініціалізація SDL2 (аналог pg.init())
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Налаштування атрибутів OpenGL (аналог pg.display.gl_set_attribute)
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core); // Використання Core-профіль OpenGL
        gl_attr.set_context_version(3, 3);            // Версія OpenGL 3.3
        gl_attr.set_depth_size(24);                   // Розмір буфера глибини
        gl_attr.set_double_buffer(true);              // Увімкнення подвійного буфера (аналог DOUBLEBUF)

        // Створення вікна (аналог pg.display.set_mode)
        let window = video_subsystem
            .window("Voxel Engine", WIN_RES.x as u32, WIN_RES.y as u32)
            .opengl()           // Увімкнення OpenGL для вікна
            .position_centered() // Центрування вікна
            .build()
            .unwrap();

        // Ініціалізація контексту OpenGL (аналог mgl.create_context())
        let _gl_context = window.gl_create_context().unwrap();

        // Завантаження функцій OpenGL (потрібно для подальшого використання, хоча тут ми використовуємо SDL2 для базового рендерингу)
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        // Налаштування параметрів рендерингу (аналог ctx.enable)
        unsafe {
            gl::Enable(gl::DEPTH_TEST); // Тест глибини
            gl::Enable(gl::CULL_FACE);  // Відсікання невидимих граней
            gl::Enable(gl::BLEND);      // Увімкнення змішування кольорів
            gl::Viewport(0, 0, WIN_RES.x as i32, WIN_RES.y as i32); // Область перегляду
        }

        // Ініціалізація обробника подій (аналог для pg.event.get())
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            sdl_context,
            window,
            last_frame: Instant::now(), // Початковий час для delta_time
            frame_count: 0,             // Ініціалізація лічильника кадрів
            fps_timer: 0.0,             // Ініціалізація таймера FPS
            is_running: true,           // Прапорець запуску
            event_pump,
        }
    }

    fn update(&mut self) {
        // Оновлення часу та FPS (аналог clock.tick() і get_ticks())
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame).as_secs_f32(); // Обчислення часу між кадрами
        self.last_frame = now;

        // Накопичення кадрів і часу для FPS
        self.frame_count += 1;
        self.fps_timer += delta_time;

        // Оновлення заголовка з FPS кожну секунду (аналог pg.display.set_caption)
        if self.fps_timer >= 1.0 {
            let fps = self.frame_count as f32 / self.fps_timer; // Обчислення FPS
            self.window
                .set_title(&format!("Voxel Engine - {:.0} FPS", fps))
                .unwrap();
            self.frame_count = 0; // Скидання лічильника
            self.fps_timer = 0.0; // Скидання таймера
        }
    }

    fn render(&mut self) {
        // Очищення екрану заданим кольором (аналог ctx.clear(color=BG_COLOR))
        unsafe {
            gl::ClearColor(BG_COLOR.x, BG_COLOR.y, BG_COLOR.z, 1.0); // Встановлення кольору очищення
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);  // Очищення буферів кольору та глибини
        }
        // Перемикання буферів (аналог pg.display.flip())
        self.window.gl_swap_window();
    }

    fn handle_events(&mut self) {
        // Обробка подій (аналог for event in pg.event.get())
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false, // Закриття при натисканні хрестика
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.is_running = false; // Закриття при натисканні Escape
                }
                _ => (),
            }
        }
    }

    fn run(&mut self) {
        // Головний цикл програми (аналог while self.is_running)
        while self.is_running {
            self.handle_events(); // Обробка подій
            self.update();        // Оновлення стану
            self.render();        // Рендеринг кадру
        }
        // Завершення програми не потребує явного виклику sys.exit(), Rust автоматично завершує процес
    }
}

fn main() {
    // Точка входу програми (аналог if __name__ == '__main__')
    let mut app = VoxelEngine::new();
    app.run();
}