mod settings;

use crate::settings::{BG_COLOR, WIN_RES};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::{GLProfile, Window};
use std::time::Instant;

struct VoxelEngine {
    sdl_context: sdl2::Sdl,
    window: Window,
    gl_context: sdl2::video::GLContext, // Додаємо поле для контексту
    last_frame: Instant,
    frame_count: u32,
    fps_timer: f32,
    is_running: bool,
    event_pump: sdl2::EventPump,
}

impl VoxelEngine {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_depth_size(24);
        gl_attr.set_double_buffer(true);

        let window = video_subsystem
            .window("Voxel Engine", WIN_RES.x as u32, WIN_RES.y as u32)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap(); // Зберігаємо контекст
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        // Налаштування OpenGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::Viewport(0, 0, WIN_RES.x as i32, WIN_RES.y as i32);
        }

        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            sdl_context,
            window,
            gl_context, // Додаємо до структури
            last_frame: Instant::now(),
            frame_count: 0,
            fps_timer: 0.0,
            is_running: true,
            event_pump,
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        self.frame_count += 1;
        self.fps_timer += delta_time;

        if self.fps_timer >= 1.0 {
            let fps = self.frame_count as f32 / self.fps_timer;
            self.window
                .set_title(&format!("Voxel Engine - {:.0} FPS", fps))
                .unwrap();
            self.frame_count = 0;
            self.fps_timer = 0.0;
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(BG_COLOR.x, BG_COLOR.y, BG_COLOR.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.window.gl_swap_window();
    }

    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_running = false;
                }
                _ => (),
            }
        }
    }

    fn run(&mut self) {
        while self.is_running {
            self.handle_events();
            self.update();
            self.render();
        }
    }
}

fn main() {
    let mut app = VoxelEngine::new();
    app.run();
}
