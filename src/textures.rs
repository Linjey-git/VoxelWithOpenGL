use image::{DynamicImage, GenericImageView};
use sdl2::video::GLContext;

pub struct Textures {
    texture_0: u32,
}

impl Textures {
    pub fn new(_gl_context: &GLContext) -> Self {
        let img = image::open("src/assets/frame.png").expect("Failed to load texture");
        let (width, height) = img.dimensions();
        let data = img.to_rgba8().into_raw();

        let mut texture_0 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_0);
            gl::BindTexture(gl::TEXTURE_2D, texture_0);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_0);
        }

        Self { texture_0 }
    }
}