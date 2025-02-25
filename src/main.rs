mod camera;
mod meshes;
mod player;
mod scene;
mod settings;
mod shader_program;
mod textures;
mod world_objects;

use crate::player::Player;
use crate::scene::Scene;
use crate::settings::Settings;
use crate::shader_program::ShaderProgram;
use crate::textures::Textures;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{GLContext, GLProfile, Window};
use std::time::Instant;

struct VoxelEngine {
    sdl_context: sdl2::Sdl,
    window: Window,
    gl_context: GLContext,
    event_pump: sdl2::EventPump,
    clock: Instant,
    delta_time: f32,
    time: f32,
    is_running: bool,
    settings: Settings,
    player: Player,
    shader_program: ShaderProgram,
    scene: Scene,
    textures: Textures,
}

impl VoxelEngine {
    fn new() -> Self {
        let settings = Settings::new();
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_depth_size(24);
        gl_attr.set_double_buffer(true);

        let window = video_subsystem
            .window(
                "Voxel Engine",
                settings.win_res.x as u32,
                settings.win_res.y as u32,
            )
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::Viewport(0, 0, settings.win_res.x as i32, settings.win_res.y as i32);
        }

        let event_pump = sdl_context.event_pump().unwrap();
        sdl_context.mouse().set_relative_mouse_mode(true); // Захоплення миші
        sdl_context.mouse().show_cursor(false); // Приховування курсора

        let textures = Textures::new(&gl_context);
        let player = Player::new(&settings);
        let shader_program = ShaderProgram::new(&player);
        let scene = Scene::new(&shader_program);

        Self {
            sdl_context,
            window,
            gl_context,
            event_pump,
            clock: Instant::now(),
            delta_time: 0.0,
            time: 0.0,
            is_running: true,
            settings,
            player,
            shader_program,
            scene,
            textures
        }
    }

    fn update(&mut self) {
        self.player
            .update(&mut self.event_pump, self.delta_time, &self.settings);
        self.shader_program.update(&self.player);
        self.scene.update();

        let now = Instant::now();
        self.delta_time = now.duration_since(self.clock).as_secs_f32() * 1000.0; // У мілісекундах
        self.clock = now;
        self.time = now.elapsed().as_secs_f32();

        let fps = 1.0 / (self.delta_time / 1000.0); // FPS
        self.window
            .set_title(&format!("Voxel Engine - {:.0} FPS", fps))
            .unwrap();
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(
                self.settings.bg_color.x,
                self.settings.bg_color.y,
                self.settings.bg_color.z,
                1.0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.scene.render();
        self.window.gl_swap_window();
    }

    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
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
