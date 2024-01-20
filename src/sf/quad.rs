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
        let verts = vec![
            // positions       // colors        // texture coords
             0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
             0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
            -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
        ];
        let inds = vec![
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];
        let image = "path/to/image.png"; // Vec<u8>

        let buf = RVertexBufferTextured::new((&verts, &inds, &image));

        Self {
            buf,
        }
    }

    // pub fn clear(&mut self) {
    //     self.buf.clear();
    // } 
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
