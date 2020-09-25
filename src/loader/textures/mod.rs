use crate::vxl_gl::Gl;

use super::Loader;

pub mod texture;

pub struct TextureLoader<'a> {
    gl: &'a Gl,
    loader: Loader,
    textures: std::collections::HashMap<&'static str, texture::Texture>,
}

impl<'a> TextureLoader<'a> {
    pub fn new(gl: &'a Gl) -> Self {
        let loader = Loader::new("images/");
        TextureLoader {
            gl,
            loader,
            textures: std::collections::HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, path: &'static str, texture_name: &'static str) -> &mut Self {
        let dyn_image = self.loader.load_as_image(path);

        let img = match dyn_image {
            image::DynamicImage::ImageRgba8(img) => img,
            img => img.to_rgba(),
        };

        let dim_touple = img.dimensions();
        let dimensions = cgmath::vec2(dim_touple.0, dim_touple.1);
        let img_data = img.into_raw();

        let tex_id = self.gl.create_texture();
        self.gl.bind_texture(tex_id);

        self.gl.set_texture_data(dimensions, img_data);
        self.gl.generate_mipmap();
        self.gl.unbind_texture();

        self.textures
            .entry(texture_name)
            .or_insert(texture::Texture::new(dimensions, tex_id));

        self
    }

    pub fn finish(&mut self) -> TextureManager {
        let mut textures: std::collections::HashMap<&'static str, texture::Texture> =
            std::collections::HashMap::new();

        self.textures.iter().for_each(|(key, value)| {
            textures.entry(key).or_insert(texture::Texture::from(value));
        });
        TextureManager::new(textures)
    }
}

pub struct TextureManager {
    textures: std::collections::HashMap<&'static str, texture::Texture>,
}

impl TextureManager {
    pub fn new(textures: std::collections::HashMap<&'static str, texture::Texture>) -> Self {
        TextureManager { textures }
    }

    pub fn get_texture(&self, texture_name: &'static str) -> &texture::Texture {
        &self.textures.get(texture_name).unwrap()
    }
}
