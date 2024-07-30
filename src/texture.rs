use bevy_ecs::system::Resource;
use glam::IVec4;
use image::{self, GenericImageView};

use tracing::info;
#[derive(Resource)]
pub struct Texture {
    pub id: gl::types::GLuint,
    pub data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub size: (u32, u32)
}

impl Default for Texture {
    fn default() -> Self {
        let tex = Self::new("world.png").unwrap();
        tex.add_to_unit(0);
        tex
    }
}

impl Texture {
    pub fn new(texpath: &'static str) -> Result<Texture, String> {
        let mut id = 0;
        let img = match image::open(texpath) {
            Ok(img) => img,
            Err(e) => return Err(format!("Failed to load texture {}", e)),
        };
        let (width, height) = img.dimensions();
        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                info!("OpenGL Error after creating texture: {}", error);
            }
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                info!("OpenGL Error after texture params: {}", error);
            }
            gl::TextureStorage2D(id, 1, gl::RGBA8, width as i32, height as i32); // Optionally create storage first
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                info!("OpenGL Error after creating texture storage: {}", error);
            }
            let data: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8().clone();
            

            gl::TextureSubImage2D(
                id,
                0,
                0,
                0,
                width as i32,
                height as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_flat_samples().as_slice().as_ptr() as *const gl::types::GLvoid,
            );
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                info!("OpenGL Error after texture subbing: {}", error);
            }
            Ok(Texture {
                id,
                data,
                size: (width, height)
            })
        }
        
    }

    pub fn add_to_unit(&self, unit: u32) {
        unsafe {
            gl::BindTextureUnit(unit as u32, self.id);
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                info!("OpenGL Error after binding texture unit: {}", error);
            }
        }
    }
}
