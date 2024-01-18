use gl::*;
use crate::sf::RVertexBufferTextured;
use crate::sf::Buffer;
use crate::sf::Drawable;

pub const T_QUAD_VS: &str = r#""#; // TODO!!!
pub const T_QUAD_FS: &str = r#""#; // TODO!!!

#[derive(Copy, Clone, Debug)]
pub struct TexturedQuad {
    buf: RVertexBufferTextured,
}

impl TexturedQuad {
    pub fn new() -> Self {
        let verts = vec![];
        let inds = vec![];
        let image = &[0]; // &[u8]

        let buf = RVertexBufferTextured::new((&verts, &inds, image));

        Self {
            buf,
        }
    }
}


impl Drawable for TexturedQuad {
    unsafe fn draw(&self, shader: &super::Shader) {
        shader.use_shader();
        // pass uniforms
        BindVertexArray(self.buf.vao_id);
        DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
        BindVertexArray(0);
    }
}
